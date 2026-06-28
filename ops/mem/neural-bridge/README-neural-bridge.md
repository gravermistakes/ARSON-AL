# ruv-FANN Neural Bridge

High-performance WebAssembly neural network library optimized for real-time inference and multi-agent systems.

## 🚀 Overview

The ruv-FANN Neural Bridge is a high-performance WASM neural network library designed for web environments with SIMD acceleration and memory-efficient agent management. It provides optimized neural network inference with strict performance targets for production AI applications.

## ✨ Features

- **🏃 SIMD Optimization**: Leverages WebAssembly SIMD for 2-4x performance improvements
- **💾 Memory Efficiency**: Advanced memory pooling and compression techniques
- **📊 Performance Monitoring**: Real-time metrics and alerting system
- **🔄 Adaptive Optimization**: Runtime performance tuning and neural architecture optimization
- **🤖 Agent Management**: Efficient spawning and lifecycle management for 25+ concurrent agents
- **🌐 Cross-Platform**: Works in any environment that supports WebAssembly

## 🎯 Performance Targets

| Metric | Target | Description |
|--------|--------|-------------|
| Agent spawn time | <75ms | Time to initialize a new neural agent |
| Neural inference | <100ms (95th percentile) | Inference latency for neural networks |
| Memory per agent | <50MB | Memory footprint per active agent |
| System health | >95% uptime | Overall system availability |
| Total memory | <2GB for 25+ agents | Memory efficiency at scale |

## 🛠 Installation

### Prerequisites

- Rust 1.70+ with `wasm32-unknown-unknown` target
- wasm-pack for building WebAssembly modules
- Node.js 16+ (for examples and testing)

### Building

```bash
# Install wasm-pack if not already installed
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build for release
wasm-pack build --target web --out-dir pkg

# Build for development
wasm-pack build --target web --dev --out-dir pkg-dev
```

## 🚀 Quick Start

### Rust API

```rust
use ruv_fann_neural_bridge::WASMNeuralBridge;

// Initialize the neural bridge
let mut bridge = WASMNeuralBridge::new(Some(25), Some(2048))?; // 25 agents, 2GB limit

// Create a neural network
bridge.create_network("classifier", &[784, 128, 64, 10], "relu,relu,sigmoid")?;

// Spawn an agent
let agent_id = bridge.spawn_agent("classifier", None)?;

// Run inference
let input = vec![0.5; 784]; // Example input
let output = bridge.run_inference(&agent_id, &input)?;

// Get performance metrics
let metrics = bridge.get_performance_metrics();
println!("Average inference time: {:.2}ms", metrics.avg_inference_time_ms);
```

### JavaScript/TypeScript API

```javascript
import init, { WASMNeuralBridge } from './pkg/ruv_fann_neural_bridge.js';

// Initialize WASM module
await init();

// Create bridge instance
const bridge = new WASMNeuralBridge(25, 2048); // 25 agents, 2GB limit

// Create neural network
bridge.create_network("classifier", [784, 128, 64, 10], "relu,relu,sigmoid");

// Spawn agent and run inference
const agentId = bridge.spawn_agent("classifier");
const input = new Float32Array(784).fill(0.5);
const output = bridge.run_inference(agentId, input);

// Monitor performance
const metrics = bridge.get_performance_metrics();
console.log(`Inference time: ${metrics.avg_inference_time_ms}ms`);
```

## 📋 Architecture

### Core Components

```
ruv-fann-neural-bridge/
├── src/
│   ├── lib.rs                 # Main library entry point
│   ├── wasm/
│   │   ├── mod.rs            # WASM module exports
│   │   └── bridge.rs         # WASM bridge implementation
│   ├── neural/
│   │   ├── mod.rs            # Neural network module
│   │   └── optimizer.rs      # Network optimization algorithms
│   └── performance/
│       ├── mod.rs            # Performance monitoring module
│       └── monitor.rs        # Real-time metrics collection
├── Cargo.toml                # Rust package configuration
├── wasm-build.toml          # WASM build configuration
└── README.md                # This file
```

### Key Modules

- **WASM Bridge** (`wasm/bridge.rs`): WebAssembly bindings and JavaScript interface
- **Neural Optimizer** (`neural/optimizer.rs`): Advanced optimization algorithms for neural networks
- **Performance Monitor** (`performance/monitor.rs`): Real-time performance tracking and alerting

## 🔧 Configuration

### Cargo Features

```toml
[features]
default = ["simd", "performance-monitoring"]

# Core features
simd = []                    # Enable SIMD optimizations
performance-monitoring = []  # Real-time performance tracking
neural-optimization = []     # Advanced neural optimizations
memory-pooling = []         # Memory pool management
batch-inference = []        # Batch processing support

# Development features
debug-logging = []          # Debug output
benchmark-mode = []         # Benchmarking utilities

# Optimization features
quantization = []           # Model quantization
pruning = []               # Network pruning
knowledge-distillation = [] # Knowledge transfer
```

### Performance Tuning

The library includes several optimization profiles:

- **Release**: Maximum optimization for production (`opt-level = "s"`, LTO enabled)
- **Dev**: Balanced performance for development (`opt-level = 1`)
- **Bench**: Optimized for benchmarking with debug info

## 📊 Performance Monitoring

### Metrics Collection

The library automatically tracks:

- **Agent Lifecycle**: Spawn times, termination times, active count
- **Neural Inference**: Latency, accuracy, batch processing metrics
- **Memory Usage**: Per-agent memory, total system memory, fragmentation
- **System Health**: CPU utilization, WASM heap usage, GC frequency

### Real-time Alerts

Configurable alerts for:
- Inference latency exceeding thresholds
- Memory usage limits
- Agent spawn failures
- System performance degradation

### Example Monitoring

```rust
use ruv_fann_neural_bridge::PerformanceMonitor;

let mut monitor = PerformanceMonitor::new();

// Record metrics during operation
monitor.record_agent_spawn("agent_1", 45.0); // 45ms spawn time
monitor.record_inference("agent_1", "network_1", 75.0, 784, 10, true, 1024);

// Check for alerts
let alerts = monitor.get_active_alerts();
for alert in alerts {
    match alert.severity {
        AlertSeverity::Critical => println!("🚨 CRITICAL: {}", alert.message),
        AlertSeverity::Warning => println!("⚠️  WARNING: {}", alert.message),
        _ => println!("ℹ️  INFO: {}", alert.message),
    }
}
```

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run benchmarks
cargo bench

# Run WASM tests
wasm-pack test --chrome --headless

# Performance tests
cargo test --features benchmark-mode --release
```

## 🚀 Deployment

### Web Applications

1. Build the WASM package:
   ```bash
   wasm-pack build --target web --out-dir pkg
   ```

2. Import in your web application:
   ```javascript
   import init, { WASMNeuralBridge } from './pkg/ruv_fann_neural_bridge.js';
   ```

### Node.js Applications

1. Build for Node.js:
   ```bash
   wasm-pack build --target nodejs --out-dir pkg-node
   ```

2. Import in Node.js:
   ```javascript
   const { WASMNeuralBridge } = require('./pkg-node/ruv_fann_neural_bridge');
   ```

## 🤝 Contributing

We welcome contributions! Please see our contributing guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes and add tests
4. Ensure all tests pass (`cargo test`)
5. Run benchmarks to verify performance (`cargo bench`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Development Setup

```bash
# Clone the repository
git clone https://github.com/ruvnet/ruv-fann-neural-bridge.git
cd ruv-fann-neural-bridge

# Install dependencies
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

# Run tests
cargo test --all-features

# Build examples
cd examples && npm install && npm run build
```

## 📖 Examples

See the `examples/` directory for:

- **Basic Usage**: Simple neural network creation and inference
- **Performance Monitoring**: Real-time metrics collection
- **Multi-Agent System**: Managing multiple neural agents
- **Web Integration**: Browser-based AI applications
- **Optimization**: Advanced neural network optimization

## 🔍 Benchmarks

Performance benchmarks are available in the `benches/` directory:

- `neural_inference.rs`: Inference latency benchmarks
- `simd_operations.rs`: SIMD operation performance
- `memory_management.rs`: Memory allocation and deallocation

Run benchmarks with:
```bash
cargo bench
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built on the ruv-FANN neural network library
- Optimized for WebAssembly and SIMD performance
- Designed for real-time AI applications

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/ruvnet/ruv-fann-neural-bridge/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ruvnet/ruv-fann-neural-bridge/discussions)
- **Documentation**: [docs.rs](https://docs.rs/ruv-fann-neural-bridge)

---

**Built with ❤️ for high-performance neural computing**