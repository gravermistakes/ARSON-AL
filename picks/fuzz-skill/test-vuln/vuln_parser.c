/*
 * Vulnerable message parser — contains real memory corruption bugs.
 * Used as test material for the fuzzing-for-memory-bugs skill baseline.
 *
 * Bugs present:
 *   1. Heap OOB write in parse_message_body (missing bounds check)
 *   2. Use-after-free in process_transaction (msg freed then reused)
 *   3. Integer overflow in calculate_buffer_size (wraps to small value)
 *   4. Stack buffer overflow in parse_header_field (unchecked sprintf)
 *   5. Off-by-one in copy_message_id (wrong size in memcpy)
 *   6. Double-free in error path of handle_message
 *
 * Build: gcc -g -fsanitize=address,undefined -o vuln_parser vuln_parser.c
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <assert.h>

#define HEADER_SIZE 64
#define MAX_FIELDS 32
#define MAX_BODY   256

typedef struct {
    char     msg_id[16];
    uint32_t body_length;
    uint32_t field_count;
    char     reserved[40];
} message_header_t;

typedef struct {
    uint32_t tag;
    uint32_t length;
    char     value[32];
} header_field_t;

typedef struct {
    message_header_t header;
    header_field_t  *fields;
    char            *body;
    int              processed;
} message_t;

/* ─── Bug 1: Heap OOB write — no check that body_length ≤ allocated body ─── */
static int parse_message_body(message_t *msg, const uint8_t *data, size_t data_len) {
    if (data_len < sizeof(message_header_t)) return -1;

    memcpy(&msg->header, data, sizeof(message_header_t));
    size_t offset = sizeof(message_header_t);

    /* Fields */
    msg->fields = (header_field_t *)calloc(msg->header.field_count, sizeof(header_field_t));
    if (!msg->fields) return -1;

    for (uint32_t i = 0; i < msg->header.field_count && offset < data_len; i++) {
        if (offset + sizeof(uint32_t) * 2 > data_len) break;
        msg->fields[i].tag    = *(uint32_t *)(data + offset); offset += 4;
        msg->fields[i].length = *(uint32_t *)(data + offset); offset += 4;
        size_t copy_len = msg->fields[i].length < 32 ? msg->fields[i].length : 32;
        if (offset + copy_len > data_len) break;
        memcpy(msg->fields[i].value, data + offset, copy_len);
        offset += copy_len;
    }

    /* Body — VULNERABILITY: body_length from attacker-controlled header,
       no validation against actual remaining data or allocation size       */
    msg->body = (char *)malloc(MAX_BODY);
    if (!msg->body) { free(msg->fields); return -1; }

    memcpy(msg->body, data + offset, msg->header.body_length); /* OOB WRITE */
    msg->body[msg->header.body_length] = '\0';                 /* OOB WRITE +1 */

    return 0;
}

/* ─── Bug 2: Use-after-free in process_transaction ─── */
static int process_transaction(message_t *msg) {
    char *body_ref = msg->body; /* save reference */

    /* Simulate processing that may free on certain conditions */
    if (msg->header.body_length > 200) {
        free(msg->body);
        msg->body = NULL;
        /* ... continue with other work ... */
    }

    /* VULNERABILITY: body_ref still points to freed memory */
    if (body_ref && body_ref[0] == 'E') {
        printf("Emergency message: %s\n", body_ref); /* UAF READ */
        if (strlen(body_ref) > 10) {
            body_ref[10] = '!';                      /* UAF WRITE */
        }
    }

    msg->processed = 1;
    return 0;
}

/* ─── Bug 3: Integer overflow in calculate_buffer_size ─── */
static size_t calculate_buffer_size(uint32_t count, uint32_t elem_size) {
    /* VULNERABILITY: count * elem_size can overflow uint32_t,
       wrapping to a small value that passes size checks */
    return (size_t)(count * elem_size);
}

static void *allocate_element_array(uint32_t count, uint32_t elem_size) {
    size_t total = calculate_buffer_size(count, elem_size);
    if (total == 0 || total > 65536) return NULL; /* guard easily bypassed */

    void *buf = malloc(total);
    if (buf) memset(buf, 0, total);
    return buf;
}

/* ─── Bug 4: Stack buffer overflow in parse_header_field ─── */
static void parse_header_field(const char *input, char *output, size_t out_sz) {
    char temp[64];

    /* VULNERABILITY: unchecked sprintf into stack buffer */
    sprintf(temp, "hdr:%s", input); /* input could be arbitrarily long */

    /* Safe copy after overflow already happened */
    size_t len = strlen(temp);
    if (len >= out_sz) len = out_sz - 1;
    memcpy(output, temp, len);
    output[len] = '\0';
}

/* ─── Bug 5: Off-by-one in copy_message_id ─── */
static void copy_message_id(char *dst, const char *src) {
    /* VULNERABILITY: should be 16, writes 17 bytes (16 + null = off-by-one) */
    memcpy(dst, src, 17);
    dst[16] = '\0';
}

/* ─── Bug 6: Double-free in handle_message error path ─── */
static int handle_message(const uint8_t *data, size_t data_len) {
    message_t *msg = (message_t *)calloc(1, sizeof(message_t));
    if (!msg) return -1;

    if (parse_message_body(msg, data, data_len) != 0) {
        /* VULNERABILITY: msg->fields or msg->body may have been allocated
           before parse failure, but only msg is freed — field/body leak */
        free(msg);
        return -1;
    }

    process_transaction(msg);

    /* VULNERABILITY: double-free — msg->body may have been freed
       inside process_transaction() already */
    free(msg->body);

    /* Also: msg->fields might not be freed if body_length triggered
       the free path (use-after-free of fields possible) */
    free(msg->fields);
    free(msg);
    return 0;
}

/* ─── Fuzzing harness (libFuzzer-style) ─── */
int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    if (size < HEADER_SIZE) return 0; /* too short, skip */
    handle_message(data, size);
    return 0;
}

/* ─── Standalone entry point for AFL / manual testing ─── */
#ifdef STANDALONE
int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s <input_file>\n", argv[0]);
        return 1;
    }

    FILE *f = fopen(argv[1], "rb");
    if (!f) { perror("fopen"); return 1; }

    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    fseek(f, 0, SEEK_SET);

    uint8_t *data = (uint8_t *)malloc(sz);
    fread(data, 1, sz, f);
    fclose(f);

    int ret = LLVMFuzzerTestOneInput(data, sz);
    free(data);
    return ret;
}
#endif

/* ── Additional vulnerable utility functions ── */

/* Bug 7: Format string vulnerability */
static void log_message(const char *user_input) {
    /* VULNERABILITY: user-controlled format string */
    printf(user_input);
}

/* Bug 8: Null pointer dereference after allocation failure */
static char *duplicate_and_extend(const char *src, size_t extra) {
    size_t len = strlen(src);
    char *buf = (char *)malloc(len + extra + 1);
    /* VULNERABILITY: no NULL check on malloc return */
    memcpy(buf, src, len);
    memset(buf + len, 'X', extra);
    buf[len + extra] = '\0';
    return buf;
}

/* Bug 9: Signed/unsigned confusion in bounds check */
static int read_at_index(char *buf, int index, size_t buf_size) {
    /* VULNERABILITY: negative index passes size check (wraps to huge size_t) */
    if ((size_t)index >= buf_size) return -1;
    return (int)buf[index]; /* reads at negative offset */
}
