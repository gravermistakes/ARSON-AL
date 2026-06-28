#include <opack/core.hpp>
#include <openssl/evp.h>
#include <cstdint>
#include <cstring>
#include <ctime>
#include <string>
#include <vector>
#include <functional>

// --- UUID: SHAKE256 → septal (base 7) → 13 digits ---

static std::string shake256_hex(const void* data, size_t len)
{
	EVP_MD_CTX* ctx = EVP_MD_CTX_new();
	EVP_DigestInit_ex(ctx, EVP_shake256(), nullptr);
	EVP_DigestUpdate(ctx, data, len);
	unsigned char out[32];
	unsigned int outlen = 32;
	EVP_DigestFinalXOF(ctx, out, outlen);
	EVP_MD_CTX_free(ctx);

	std::string hex;
	hex.reserve(64);
	for (unsigned i = 0; i < outlen; i++)
	{
		char buf[3];
		snprintf(buf, sizeof(buf), "%02x", out[i]);
		hex += buf;
	}
	return hex;
}

static std::string to_septal(const unsigned char* bytes, size_t len, int digits)
{
	std::string result;
	result.reserve(digits);
	size_t bit_pos = 0;
	while ((int)result.size() < digits && bit_pos + 3 <= len * 8)
	{
		size_t byte_idx = bit_pos / 8;
		size_t bit_off = bit_pos % 8;
		uint16_t window = (uint16_t)bytes[byte_idx] << 8;
		if (byte_idx + 1 < len) window |= bytes[byte_idx + 1];
		int val = (window >> (13 - bit_off)) & 0x07;
		if (val < 7)
		{
			result += ('0' + val);
		}
		bit_pos += 3;
	}
	while ((int)result.size() < digits) result += '0';
	return result;
}

static std::string gen_uuid(uint64_t timestamp)
{
	EVP_MD_CTX* ctx = EVP_MD_CTX_new();
	EVP_DigestInit_ex(ctx, EVP_shake256(), nullptr);
	EVP_DigestUpdate(ctx, &timestamp, sizeof(timestamp));
	unsigned char hash[32];
	EVP_DigestFinalXOF(ctx, hash, 32);
	EVP_MD_CTX_free(ctx);
	return to_septal(hash, 32, 13);
}

// --- Deterministic PRNG seeded from UUID ---

struct UuidPrng
{
	uint64_t state;

	explicit UuidPrng(const std::string& uuid)
	{
		state = 0;
		for (char c : uuid)
			state = state * 7 + (c - '0');
		if (state == 0) state = 1;
	}

	uint64_t next()
	{
		state ^= state << 13;
		state ^= state >> 7;
		state ^= state << 17;
		return state;
	}

	int pick(int n) { return (int)(next() % (uint64_t)n); }
};

// --- ECS Components ---

struct ActorUuid { std::string value; };
struct ActorScore { int64_t cumulative; int level; };
struct ActorRep { int64_t value; int rank; };
struct ActorSeed { UuidPrng prng; };

struct KitName { std::string value; };
struct KitTools { std::vector<std::string> tools; };

struct Finding
{
	enum Type { Surface, Vuln, Exploit, Chain, Report };
	Type type;
	std::string description;
	std::string cwe;
	float cvss;
};

struct Target { std::string scope; };
struct HuntState { int tick; bool active; };

// --- Kit switch rules (deterministic, compiled) ---

enum class KitId { Recon, Scan, Exploit, Validate, Report, COUNT };

static const char* kit_names[] = {"recon", "scan", "exploit", "validate", "report"};

static KitId switch_rule(KitId current, Finding::Type finding_type)
{
	switch (finding_type)
	{
		case Finding::Surface:       return KitId::Scan;
		case Finding::Vuln:          return KitId::Exploit;
		case Finding::Exploit:       return KitId::Validate;
		case Finding::Chain:         return KitId::Report;
		case Finding::Report:        return KitId::Recon;
	}
	return current;
}

// --- Simulated tool execution (deterministic, seeded) ---

static Finding run_tool(KitId kit, UuidPrng& prng, int tick)
{
	Finding f;
	int roll = prng.pick(100);

	switch (kit)
	{
		case KitId::Recon:
			f.type = Finding::Surface;
			f.description = "subdomain-" + std::to_string(prng.pick(1000));
			f.cwe = "";
			f.cvss = 0.0f;
			break;
		case KitId::Scan:
			if (roll < 30) {
				f.type = Finding::Vuln;
				f.description = "sqli-endpoint-" + std::to_string(prng.pick(50));
				f.cwe = "CWE-89";
				f.cvss = 7.5f + (prng.pick(25)) / 10.0f;
			} else {
				f.type = Finding::Surface;
				f.description = "port-" + std::to_string(prng.pick(65535));
				f.cwe = "";
				f.cvss = 0.0f;
			}
			break;
		case KitId::Exploit:
			if (roll < 50) {
				f.type = Finding::Exploit;
				f.description = "poc-confirmed";
				f.cwe = "CWE-89";
				f.cvss = 9.1f;
			} else {
				f.type = Finding::Vuln;
				f.description = "needs-deeper-scan";
				f.cwe = "";
				f.cvss = 0.0f;
			}
			break;
		case KitId::Validate:
			if (roll < 70) {
				f.type = Finding::Chain;
				f.description = "chain-validated";
				f.cwe = "CWE-89";
				f.cvss = 9.8f;
			} else {
				f.type = Finding::Surface;
				f.description = "validation-failed-retry";
				f.cwe = "";
				f.cvss = 0.0f;
			}
			break;
		case KitId::Report:
			f.type = Finding::Report;
			f.description = "report-submitted";
			f.cwe = "";
			f.cvss = 0.0f;
			break;
		default:
			f.type = Finding::Surface;
			f.description = "unknown";
			f.cwe = "";
			f.cvss = 0.0f;
	}
	return f;
}

int main()
{
	auto world = opack::create_world();
	opack::target_fps(world, 1);

	// Spawn actor with UUID
	uint64_t timestamp = (uint64_t)std::time(nullptr);
	std::string uuid = gen_uuid(timestamp);
	UuidPrng prng(uuid);

	fmt::print("=== Opaca Actor Bootstrap ===\n");
	fmt::print("UUID: {} (septal, 13 digits)\n", uuid);
	fmt::print("Timestamp: {}\n\n", timestamp);

	// Actor state
	KitId current_kit = KitId::Recon;
	int hunt_tick = 0;
	int max_ticks = 20;
	int findings_count = 0;
	int kit_switches = 0;

	// The deterministic loop: perceive → decide → act
	// No LLM. No Agent call. Pure compiled logic.
	while (hunt_tick < max_ticks)
	{
		hunt_tick++;

		// PERCEIVE: read world state (in full system, query ECS)
		fmt::print("[tick {:>3}] kit={:<10} | ", hunt_tick, kit_names[(int)current_kit]);

		// ACT: run current kit's tool (deterministic, seeded)
		Finding f = run_tool(current_kit, prng, hunt_tick);
		findings_count++;

		fmt::print("finding: {:<30} ", f.description);
		if (f.cvss > 0.0f)
			fmt::print("(CWE={}, CVSS={:.1f}) ", f.cwe, f.cvss);

		// DECIDE: evaluate kit-switch rules
		KitId next_kit = switch_rule(current_kit, f.type);
		if (next_kit != current_kit)
		{
			fmt::print("→ switch to {}", kit_names[(int)next_kit]);
			current_kit = next_kit;
			kit_switches++;
		}
		fmt::print("\n");

		// Step the ECS world
		opack::step(world);
	}

	fmt::print("\n=== Hunt Complete ===\n");
	fmt::print("UUID:          {}\n", uuid);
	fmt::print("Ticks:         {}\n", hunt_tick);
	fmt::print("Findings:      {}\n", findings_count);
	fmt::print("Kit switches:  {}\n", kit_switches);
	fmt::print("Final kit:     {}\n", kit_names[(int)current_kit]);
	fmt::print("\nDeterministic: same UUID + same timestamp = identical run.\n");

	return 0;
}
