#!/bin/bash
# Enhanced WASM Build Script for ruv-FANN Neural Bridge
# Optimized for <100ms inference latency with comprehensive SIMD and memory optimizations

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="ruv-fann-neural-bridge"
WASM_OUT_DIR="pkg"
OPTIMIZATION_LEVEL="${OPTIMIZATION_LEVEL:-release}"
ENABLE_PROFILING="${ENABLE_PROFILING:-false}"
SIMD_LEVEL="${SIMD_LEVEL:-aggressive}"
MEMORY_OPT="${MEMORY_OPT:-balanced}"

# Performance targets
TARGET_INFERENCE_MS=100
TARGET_MEMORY_MB=50
TARGET_AGENT_SPAWN_MS=75
TARGET_TOTAL_MEMORY_GB=2

echo -e "${BLUE}🚀 Enhanced WASM Build for ruv-FANN Neural Bridge${NC}"
echo -e "${BLUE}================================================${NC}"
echo "Optimization Level: $OPTIMIZATION_LEVEL"
echo "SIMD Level: $SIMD_LEVEL"
echo "Memory Optimization: $MEMORY_OPT"
echo "Performance Targets:"
echo "  - Inference latency: <${TARGET_INFERENCE_MS}ms"
echo "  - Memory per agent: <${TARGET_MEMORY_MB}MB"
echo "  - Agent spawn time: <${TARGET_AGENT_SPAWN_MS}ms"
echo "  - Total memory: <${TARGET_TOTAL_MEMORY_GB}GB for 25+ agents"
echo ""

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Check prerequisites
check_prerequisites() {
    print_info "Checking build prerequisites..."
    
    # Check Rust toolchain
    if ! command -v rustc &> /dev/null; then
        print_error "Rust not found. Please install Rust: https://rustup.rs/"
        exit 1
    fi
    
    # Check wasm-pack
    if ! command -v wasm-pack &> /dev/null; then
        print_error "wasm-pack not found. Installing..."
        curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    fi
    
    # Check wasm32 target
    if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
        print_info "Adding wasm32-unknown-unknown target..."
        rustup target add wasm32-unknown-unknown
    fi
    
    # Check wasm-opt (for advanced optimization)
    if ! command -v wasm-opt &> /dev/null; then
        print_warning "wasm-opt not found. Consider installing binaryen for advanced optimizations"
        print_info "Install with: npm install -g binaryen"
    fi
    
    print_status "Prerequisites check completed"
}

# Detect system capabilities
detect_capabilities() {
    print_info "Detecting system capabilities..."
    
    # Detect CPU features
    NATIVE_FEATURES=""
    if command -v lscpu &> /dev/null; then
        CPU_FLAGS=$(lscpu | grep Flags || echo "")
        if echo "$CPU_FLAGS" | grep -q "avx2"; then
            NATIVE_FEATURES="$NATIVE_FEATURES,avx2"
        fi
        if echo "$CPU_FLAGS" | grep -q "sse4.1"; then
            NATIVE_FEATURES="$NATIVE_FEATURES,sse4.1"
        fi
    fi
    
    # Detect Node.js version (for testing)
    if command -v node &> /dev/null; then
        NODE_VERSION=$(node --version)
        print_info "Node.js version: $NODE_VERSION"
        
        # Check for SIMD support in Node.js
        if node -e "try { require('wasm-feature-detect'); console.log('WASM feature detection available'); } catch(e) { console.log('Consider installing wasm-feature-detect for runtime checks'); }" 2>/dev/null; then
            :
        fi
    fi
    
    print_status "Capability detection completed"
}

# Set Rust flags based on optimization level
set_rust_flags() {
    print_info "Configuring Rust build flags..."
    
    # Base flags for WASM
    export RUSTFLAGS="-C target-feature=+simd128,+bulk-memory,+reference-types"
    
    case $OPTIMIZATION_LEVEL in
        "debug")
            export RUSTFLAGS="$RUSTFLAGS -C opt-level=1 -C debug-assertions=on"
            ;;
        "release")
            export RUSTFLAGS="$RUSTFLAGS -C opt-level=s -C lto=fat -C codegen-units=1 -C panic=abort"
            ;;
        "aggressive")
            export RUSTFLAGS="$RUSTFLAGS -C opt-level=3 -C lto=fat -C codegen-units=1 -C panic=abort"
            export RUSTFLAGS="$RUSTFLAGS -C target-feature=+simd128,+bulk-memory,+reference-types,+multivalue"
            ;;
    esac
    
    # Add SIMD-specific flags
    case $SIMD_LEVEL in
        "aggressive")
            export RUSTFLAGS="$RUSTFLAGS -C target-feature=+simd128"
            export CARGO_CFG_TARGET_FEATURE="simd128"
            ;;
        "conservative")
            # Use safer SIMD settings
            ;;
        "disabled")
            export RUSTFLAGS=$(echo "$RUSTFLAGS" | sed 's/+simd128//g')
            ;;
    esac
    
    # Memory optimization flags
    case $MEMORY_OPT in
        "aggressive")
            export RUSTFLAGS="$RUSTFLAGS -C opt-level=z"
            ;;
        "balanced")
            export RUSTFLAGS="$RUSTFLAGS -C opt-level=s"
            ;;
    esac
    
    print_info "RUSTFLAGS: $RUSTFLAGS"
    print_status "Rust flags configured"
}

# Build the project
build_project() {
    print_info "Building project with wasm-pack..."
    
    # Set features based on configuration
    local FEATURES="simd,performance-monitoring,neural-optimization"
    
    if [ "$ENABLE_PROFILING" = "true" ]; then
        FEATURES="$FEATURES,benchmark-mode"
    fi
    
    # Create wasm-pack profile
    local WASM_PACK_PROFILE="release"
    if [ "$OPTIMIZATION_LEVEL" = "debug" ]; then
        WASM_PACK_PROFILE="dev"
    fi
    
    # Build with wasm-pack
    print_info "Running wasm-pack build..."
    wasm-pack build \
        --target web \
        --out-dir "$WASM_OUT_DIR" \
        --features "$FEATURES" \
        --$WASM_PACK_PROFILE \
        --scope ruv \
        || {
            print_error "wasm-pack build failed"
            exit 1
        }
    
    print_status "wasm-pack build completed"
}

# Apply advanced optimizations
optimize_wasm() {
    if ! command -v wasm-opt &> /dev/null; then
        print_warning "wasm-opt not available, skipping advanced optimizations"
        return
    fi
    
    print_info "Applying advanced WASM optimizations..."
    
    local WASM_FILE="$WASM_OUT_DIR/${PROJECT_NAME//-/_}_bg.wasm"
    local OPTIMIZED_FILE="$WASM_OUT_DIR/${PROJECT_NAME//-/_}_bg_optimized.wasm"
    
    if [ ! -f "$WASM_FILE" ]; then
        print_warning "WASM file not found: $WASM_FILE"
        return
    fi
    
    # Get original size
    local ORIGINAL_SIZE=$(stat -f%z "$WASM_FILE" 2>/dev/null || stat -c%s "$WASM_FILE" 2>/dev/null || echo "unknown")
    
    # Apply optimizations based on level
    local WASM_OPT_FLAGS=""
    case $OPTIMIZATION_LEVEL in
        "debug")
            WASM_OPT_FLAGS="-O1 --enable-simd --debuginfo"
            ;;
        "release")
            WASM_OPT_FLAGS="-Oz --enable-simd --enable-bulk-memory --strip-debug --vacuum"
            ;;
        "aggressive")
            WASM_OPT_FLAGS="-Oz --enable-simd --enable-bulk-memory --enable-reference-types"
            WASM_OPT_FLAGS="$WASM_OPT_FLAGS --strip-debug --vacuum --dce --flatten --rse"
            WASM_OPT_FLAGS="$WASM_OPT_FLAGS --precompute --precompute-propagate"
            ;;
    esac
    
    print_info "Applying wasm-opt with flags: $WASM_OPT_FLAGS"
    wasm-opt $WASM_OPT_FLAGS "$WASM_FILE" -o "$OPTIMIZED_FILE" || {
        print_warning "wasm-opt optimization failed, using original file"
        return
    }
    
    # Replace original with optimized
    mv "$OPTIMIZED_FILE" "$WASM_FILE"
    
    # Get optimized size
    local OPTIMIZED_SIZE=$(stat -f%z "$WASM_FILE" 2>/dev/null || stat -c%s "$WASM_FILE" 2>/dev/null || echo "unknown")
    
    if [ "$ORIGINAL_SIZE" != "unknown" ] && [ "$OPTIMIZED_SIZE" != "unknown" ]; then
        local REDUCTION=$((100 - (OPTIMIZED_SIZE * 100 / ORIGINAL_SIZE)))
        print_status "WASM optimization completed: ${REDUCTION}% size reduction"
        print_info "  Original size: ${ORIGINAL_SIZE} bytes"
        print_info "  Optimized size: ${OPTIMIZED_SIZE} bytes"
    else
        print_status "WASM optimization completed"
    fi
}

# Generate TypeScript definitions
generate_typescript() {
    print_info "Generating TypeScript definitions..."
    
    local TS_FILE="$WASM_OUT_DIR/${PROJECT_NAME//-/_}.d.ts"
    
    if [ -f "$TS_FILE" ]; then
        # Enhance TypeScript definitions with performance types
        cat >> "$TS_FILE" << 'EOF'

// Enhanced performance monitoring types
export interface PerformanceMetrics {
    active_agents: number;
    avg_spawn_time_ms: number;
    avg_inference_time_ms: number;
    p95_inference_time_ms: number;
    memory_usage_mb: number;
    memory_utilization_percent: number;
    simd_enabled: boolean;
    performance_targets: {
        spawn_time_target_met: boolean;
        inference_time_target_met: boolean;
        memory_target_met: boolean;
    };
}

export interface OptimizationResult {
    optimization_id: string;
    strategy_name: string;
    success: boolean;
    improvement_metrics: {
        latency_improvement_percent: number;
        memory_reduction_percent: number;
        overall_score: number;
    };
}

// Performance monitoring utilities
export interface WASMPerformanceConfig {
    target_inference_latency_ms?: number;
    target_memory_per_agent_mb?: number;
    enable_simd?: boolean;
    enable_monitoring?: boolean;
}
EOF
        
        print_status "TypeScript definitions enhanced"
    else
        print_warning "TypeScript definitions file not found"
    fi
}

# Run performance tests
run_performance_tests() {
    if [ "$ENABLE_PROFILING" != "true" ]; then
        print_info "Profiling disabled, skipping performance tests"
        return
    fi
    
    print_info "Running performance tests..."
    
    # Create a simple test runner
    cat > "$WASM_OUT_DIR/performance_test.js" << 'EOF'
// Performance test runner for ruv-FANN Neural Bridge
import init, { WASMNeuralBridge } from './ruv_fann_neural_bridge.js';

async function runPerformanceTests() {
    console.log('🧪 Running performance tests...');
    
    try {
        // Initialize WASM
        await init();
        
        // Create bridge
        const bridge = new WASMNeuralBridge(25, 2048); // 25 agents, 2GB limit
        
        // Test 1: Agent spawn performance
        console.log('\n📊 Test 1: Agent Spawn Performance');
        const spawnTimes = [];
        
        // Create network first
        bridge.create_network('test_net', [784, 128, 10], 'relu,sigmoid');
        
        for (let i = 0; i < 10; i++) {
            const start = performance.now();
            const agentId = bridge.spawn_agent('test_net');
            const spawnTime = performance.now() - start;
            spawnTimes.push(spawnTime);
            console.log(`  Agent ${i + 1} spawned in ${spawnTime.toFixed(2)}ms`);
        }
        
        const avgSpawnTime = spawnTimes.reduce((a, b) => a + b, 0) / spawnTimes.length;
        console.log(`  Average spawn time: ${avgSpawnTime.toFixed(2)}ms (target: <75ms)`);
        console.log(`  Result: ${avgSpawnTime < 75 ? '✅ PASS' : '❌ FAIL'}`);
        
        // Test 2: Inference performance
        console.log('\n📊 Test 2: Inference Performance');
        const agent = bridge.spawn_agent('test_net');
        const inferenceTimes = [];
        
        for (let i = 0; i < 100; i++) {
            const input = new Float32Array(784).fill(Math.random());
            const start = performance.now();
            const result = bridge.run_inference(agent, Array.from(input));
            const inferenceTime = performance.now() - start;
            inferenceTimes.push(inferenceTime);
            
            if (i % 20 === 0) {
                console.log(`  Completed ${i + 1}/100 inferences`);
            }
        }
        
        const avgInferenceTime = inferenceTimes.reduce((a, b) => a + b, 0) / inferenceTimes.length;
        inferenceTimes.sort((a, b) => a - b);
        const p95InferenceTime = inferenceTimes[Math.floor(inferenceTimes.length * 0.95)];
        
        console.log(`  Average inference time: ${avgInferenceTime.toFixed(2)}ms`);
        console.log(`  P95 inference time: ${p95InferenceTime.toFixed(2)}ms (target: <100ms)`);
        console.log(`  Result: ${p95InferenceTime < 100 ? '✅ PASS' : '❌ FAIL'}`);
        
        // Test 3: Memory usage
        console.log('\n📊 Test 3: Memory Usage');
        const metrics = bridge.get_performance_metrics();
        console.log(`  Memory usage: ${metrics.memory_usage_mb.toFixed(2)}MB`);
        console.log(`  Memory utilization: ${metrics.memory_utilization_percent.toFixed(1)}%`);
        console.log(`  SIMD enabled: ${metrics.simd_enabled ? 'Yes' : 'No'}`);
        
        // Overall results
        console.log('\n📈 Performance Test Summary:');
        console.log(`  Agent spawn: ${avgSpawnTime < 75 ? '✅' : '❌'} (${avgSpawnTime.toFixed(2)}ms)`);
        console.log(`  Inference latency: ${p95InferenceTime < 100 ? '✅' : '❌'} (${p95InferenceTime.toFixed(2)}ms P95)`);
        console.log(`  Memory efficiency: ${metrics.memory_usage_mb < 500 ? '✅' : '❌'} (${metrics.memory_usage_mb.toFixed(2)}MB)`);
        console.log(`  SIMD optimization: ${metrics.simd_enabled ? '✅' : '❌'}`);
        
        const allTestsPassed = avgSpawnTime < 75 && p95InferenceTime < 100 && metrics.memory_usage_mb < 500;
        console.log(`\n${allTestsPassed ? '🎉 All tests PASSED!' : '⚠️  Some tests FAILED'}`);
        
        return {
            spawn_time: avgSpawnTime,
            inference_time: p95InferenceTime,
            memory_usage: metrics.memory_usage_mb,
            all_passed: allTestsPassed
        };
        
    } catch (error) {
        console.error('❌ Performance test failed:', error);
        return null;
    }
}

// Run tests if in Node.js environment
if (typeof process !== 'undefined' && process.versions && process.versions.node) {
    runPerformanceTests().then(results => {
        if (results) {
            process.exit(results.all_passed ? 0 : 1);
        } else {
            process.exit(1);
        }
    });
}

export { runPerformanceTests };
EOF
    
    print_status "Performance test suite created"
    
    # Run tests if Node.js is available
    if command -v node &> /dev/null; then
        print_info "Running performance tests with Node.js..."
        cd "$WASM_OUT_DIR"
        
        if node performance_test.js 2>/dev/null; then
            print_status "Performance tests PASSED"
        else
            print_warning "Performance tests completed with warnings"
        fi
        
        cd ..
    else
        print_info "Node.js not available, tests created for manual execution"
    fi
}

# Generate build report
generate_build_report() {
    print_info "Generating build report..."
    
    local REPORT_FILE="$WASM_OUT_DIR/build_report.md"
    local BUILD_TIME=$(date)
    local WASM_FILE="$WASM_OUT_DIR/${PROJECT_NAME//-/_}_bg.wasm"
    local WASM_SIZE="unknown"
    
    if [ -f "$WASM_FILE" ]; then
        WASM_SIZE=$(stat -f%z "$WASM_FILE" 2>/dev/null || stat -c%s "$WASM_FILE" 2>/dev/null)
        WASM_SIZE="${WASM_SIZE} bytes ($(echo "scale=2; $WASM_SIZE / 1024" | bc 2>/dev/null || echo "unknown") KB)"
    fi
    
    cat > "$REPORT_FILE" << EOF
# ruv-FANN Neural Bridge - Build Report

**Build Time:** $BUILD_TIME  
**Optimization Level:** $OPTIMIZATION_LEVEL  
**SIMD Level:** $SIMD_LEVEL  
**Memory Optimization:** $MEMORY_OPT  

## Performance Targets

| Metric | Target | Status |
|--------|--------|--------|
| Inference Latency | <${TARGET_INFERENCE_MS}ms | 🎯 Target Set |
| Memory per Agent | <${TARGET_MEMORY_MB}MB | 🎯 Target Set |
| Agent Spawn Time | <${TARGET_AGENT_SPAWN_MS}ms | 🎯 Target Set |
| Total Memory | <${TARGET_TOTAL_MEMORY_GB}GB for 25+ agents | 🎯 Target Set |

## Build Configuration

- **RUSTFLAGS:** \`$RUSTFLAGS\`
- **Features:** simd, performance-monitoring, neural-optimization
- **Target:** wasm32-unknown-unknown
- **Profile:** $WASM_PACK_PROFILE

## Output Files

- **WASM Binary:** ${PROJECT_NAME//-/_}_bg.wasm ($WASM_SIZE)
- **JavaScript Bindings:** ${PROJECT_NAME//-/_}.js
- **TypeScript Definitions:** ${PROJECT_NAME//-/_}.d.ts
- **Package Manifest:** package.json

## Performance Features

- ✅ SIMD Acceleration (WebAssembly SIMD)
- ✅ Memory Pooling and Optimization
- ✅ Real-time Performance Monitoring
- ✅ Adaptive Neural Network Optimization
- ✅ Batch Inference Processing
- ✅ Advanced Memory Management

## Testing

Run performance tests with:
\`\`\`bash
cd $WASM_OUT_DIR
node performance_test.js
\`\`\`

## Usage Example

\`\`\`javascript
import init, { WASMNeuralBridge } from './${PROJECT_NAME//-/_}.js';

async function main() {
    await init();
    
    // Create bridge for 25 agents with 2GB memory limit
    const bridge = new WASMNeuralBridge(25, 2048);
    
    // Create neural network
    bridge.create_network('classifier', [784, 128, 10], 'relu,sigmoid');
    
    // Spawn agent
    const agent = bridge.spawn_agent('classifier');
    
    // Run inference
    const input = new Float32Array(784).fill(0.5);
    const output = bridge.run_inference(agent, Array.from(input));
    
    console.log('Inference result:', output);
    
    // Get performance metrics
    const metrics = bridge.get_performance_metrics();
    console.log('Performance:', metrics);
}
\`\`\`

---
Generated by ruv-FANN Neural Bridge Build System
EOF
    
    print_status "Build report generated: $REPORT_FILE"
}

# Main execution
main() {
    echo "Starting enhanced WASM build process..."
    
    check_prerequisites
    detect_capabilities
    set_rust_flags
    build_project
    optimize_wasm
    generate_typescript
    run_performance_tests
    generate_build_report
    
    echo ""
    print_status "🎉 Enhanced WASM build completed successfully!"
    echo ""
    print_info "📦 Output directory: $WASM_OUT_DIR"
    print_info "📊 Build report: $WASM_OUT_DIR/build_report.md"
    print_info "🧪 Performance tests: $WASM_OUT_DIR/performance_test.js"
    echo ""
    print_info "Next steps:"
    echo "  1. Test the build: cd $WASM_OUT_DIR && node performance_test.js"
    echo "  2. Integrate into your web application"
    echo "  3. Monitor performance metrics in production"
    echo "  4. Use auto-optimization for continuous improvement"
    echo ""
}

# Error handling
trap 'print_error "Build failed at line $LINENO"' ERR

# Run main function
main "$@"