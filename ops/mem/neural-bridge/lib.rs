//! ruv-FANN Neural Bridge - High-Performance WASM Neural Network Library
//! 
//! This library provides optimized neural network inference for web environments
//! with SIMD acceleration and memory-efficient agent management.
//! 
//! # Features
//! 
//! - **SIMD Optimization**: Leverages WebAssembly SIMD for 2-4x performance improvements
//! - **Memory Efficiency**: Advanced memory pooling and compression techniques
//! - **Performance Monitoring**: Real-time metrics and alerting system
//! - **Adaptive Optimization**: Runtime performance tuning and neural architecture optimization
//! - **Agent Management**: Efficient spawning and lifecycle management for 25+ concurrent agents
//! 
//! # Performance Targets
//! 
//! - Agent spawn time: <75ms
//! - Neural inference: <100ms (95th percentile)
//! - Memory per agent: <50MB
//! - System health: >95% uptime
//! - Total memory: <2GB for 25+ agents
//! 
//! # Example Usage
//! 
//! ```rust,no_run
//! use ruv_fann_neural_bridge::WASMNeuralBridge;
//! 
//! // Initialize the neural bridge
//! let mut bridge = WASMNeuralBridge::new(Some(25), Some(2048))?; // 25 agents, 2GB limit
//! 
//! // Create a neural network
//! bridge.create_network("classifier", &[784, 128, 64, 10], "relu,relu,sigmoid")?;
//! 
//! // Spawn an agent
//! let agent_id = bridge.spawn_agent("classifier", None)?;
//! 
//! // Run inference
//! let input = vec![0.5; 784]; // Example input
//! let output = bridge.run_inference(&agent_id, &input)?;
//! 
//! // Get performance metrics
//! let metrics = bridge.get_performance_metrics();
//! println!("Average inference time: {:.2}ms", metrics.avg_inference_time_ms);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

// Module declarations
pub mod wasm;
pub mod performance;
pub mod neural;

// Re-export main types for convenience
pub use wasm::bridge::{WASMNeuralBridge, NeuralAgent};
pub use performance::monitor::{
    PerformanceMonitor, PerformanceMetrics, PerformanceThresholds,
    PerformanceSnapshot, PerformanceAlert, AlertSeverity, AlertType
};
pub use neural::optimizer::{
    NeuralOptimizer, OptimizationConfig, OptimizationStrategy,
    OptimizationResult, PerformanceProfile
};

// Error types
pub use wasm::bridge::*;

// Feature-gated exports
#[cfg(feature = "simd")]
pub use simd_config::*;

// WASM-specific exports
#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen::prelude::*;

// Utility functions and constants
pub mod utils {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    /// Current timestamp in milliseconds
    pub fn current_timestamp_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
    
    /// Generate a unique ID for agents or networks
    pub fn generate_unique_id(prefix: &str) -> String {
        format!("{}_{}_{}",
               prefix,
               current_timestamp_ms(),
               fastrand::u32(100000..999999))
    }
    
    /// Calculate percentage improvement
    pub fn calculate_improvement_percent(baseline: f64, optimized: f64) -> f64 {
        if baseline == 0.0 {
            0.0
        } else {
            ((baseline - optimized) / baseline) * 100.0
        }
    }
    
    /// Validate performance against thresholds
    pub fn check_performance_threshold(value: f64, threshold: f64, is_max_threshold: bool) -> bool {
        if is_max_threshold {
            value <= threshold
        } else {
            value >= threshold
        }
    }
}

// Performance benchmarking utilities
#[cfg(feature = "benchmark-mode")]
pub mod benchmark {
    use std::time::Instant;
    
    /// Simple benchmark wrapper
    pub struct Benchmark {
        name: String,
        start_time: Option<Instant>,
    }
    
    impl Benchmark {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                start_time: None,
            }
        }
        
        pub fn start(&mut self) {
            self.start_time = Some(Instant::now());
        }
        
        pub fn finish(&self) -> f64 {
            if let Some(start) = self.start_time {
                let duration = start.elapsed();
                let duration_ms = duration.as_millis() as f64;
                println!("Benchmark '{}': {:.2}ms", self.name, duration_ms);
                duration_ms
            } else {
                println!("Benchmark '{}': not started", self.name);
                0.0
            }
        }
    }
    
    /// Macro for easy benchmarking
    #[macro_export]
    macro_rules! bench {
        ($name:expr, $code:block) => {{
            let mut benchmark = $crate::benchmark::Benchmark::new($name);
            benchmark.start();
            let result = $code;
            benchmark.finish();
            result
        }};
    }
}

// Memory management utilities
pub mod memory {
    /// Memory usage estimation utilities
    pub struct MemoryEstimator;
    
    impl MemoryEstimator {
        /// Estimate memory usage for a neural network
        pub fn estimate_network_memory(layer_sizes: &[usize]) -> usize {
            let mut total_weights = 0;
            
            // Calculate weights between layers
            for i in 0..layer_sizes.len() - 1 {
                total_weights += layer_sizes[i] * layer_sizes[i + 1];
            }
            
            // Calculate biases
            let total_biases: usize = layer_sizes.iter().skip(1).sum();
            
            // Estimate total memory (weights + biases) * 4 bytes per f32 + overhead
            let base_memory = (total_weights + total_biases) * std::mem::size_of::<f32>();
            let overhead = base_memory / 10; // 10% overhead for structure
            
            base_memory + overhead
        }
        
        /// Estimate memory usage for an agent
        pub fn estimate_agent_memory(network_memory: usize) -> usize {
            let base_agent_overhead = 1024 * 1024; // 1MB base overhead
            network_memory + base_agent_overhead
        }
    }
}

// SIMD configuration integration
#[cfg(feature = "simd")]
mod simd_config;

// Version and build information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP", "unknown");
pub const GIT_HASH: &str = env!("GIT_HASH", "unknown");

/// Build information structure
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub version: &'static str,
    pub build_timestamp: &'static str,
    pub git_hash: &'static str,
    pub features: Vec<&'static str>,
    pub target_arch: &'static str,
    pub target_os: &'static str,
}

impl BuildInfo {
    /// Get current build information
    pub fn current() -> Self {
        Self {
            version: VERSION,
            build_timestamp: BUILD_TIMESTAMP,
            git_hash: GIT_HASH,
            features: Self::enabled_features(),
            target_arch: std::env::consts::ARCH,
            target_os: std::env::consts::OS,
        }
    }
    
    fn enabled_features() -> Vec<&'static str> {
        let mut features = Vec::new();
        
        #[cfg(feature = "simd")]
        features.push("simd");
        
        #[cfg(feature = "performance-monitoring")]
        features.push("performance-monitoring");
        
        #[cfg(feature = "neural-optimization")]
        features.push("neural-optimization");
        
        #[cfg(feature = "memory-pooling")]
        features.push("memory-pooling");
        
        #[cfg(feature = "batch-inference")]
        features.push("batch-inference");
        
        #[cfg(feature = "debug-logging")]
        features.push("debug-logging");
        
        #[cfg(feature = "benchmark-mode")]
        features.push("benchmark-mode");
        
        features
    }
}

// Global initialization for WASM
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init() {
    // Set panic hook for better error reporting
    console_error_panic_hook::set_once();
    
    // Initialize logging
    #[cfg(feature = "debug-logging")]
    {
        use web_sys::console;
        console::log_1(&format!("ruv-FANN Neural Bridge v{} initialized", VERSION).into());
        console::log_1(&format!("Features: {:?}", BuildInfo::current().features).into());
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_build_info() {
        let info = BuildInfo::current();
        assert!(!info.version.is_empty());
        assert!(!info.target_arch.is_empty());
        assert!(!info.target_os.is_empty());
    }
    
    #[test]
    fn test_memory_estimation() {
        let layer_sizes = vec![784, 128, 64, 10];
        let network_memory = memory::MemoryEstimator::estimate_network_memory(&layer_sizes);
        let agent_memory = memory::MemoryEstimator::estimate_agent_memory(network_memory);
        
        assert!(network_memory > 0);
        assert!(agent_memory > network_memory);
        
        // Should be reasonable for this network size (less than 1MB)
        assert!(network_memory < 1024 * 1024);
    }
    
    #[test]
    fn test_utility_functions() {
        let timestamp = utils::current_timestamp_ms();
        assert!(timestamp > 0);
        
        let unique_id = utils::generate_unique_id("test");
        assert!(unique_id.starts_with("test_"));
        
        let improvement = utils::calculate_improvement_percent(100.0, 80.0);
        assert_eq!(improvement, 20.0);
        
        assert!(utils::check_performance_threshold(90.0, 100.0, true));
        assert!(!utils::check_performance_threshold(110.0, 100.0, true));
    }
    
    #[cfg(feature = "benchmark-mode")]
    #[test]
    fn test_benchmarking() {
        let result = bench!("test_operation", {
            std::thread::sleep(std::time::Duration::from_millis(10));
            42
        });
        
        assert_eq!(result, 42);
    }
}

// Documentation examples
#[cfg(doc)]
mod examples {
    /// # Basic Usage Example
    /// 
    /// ```rust,no_run
    /// use ruv_fann_neural_bridge::{WASMNeuralBridge, utils};
    /// 
    /// // Create a bridge for up to 25 agents with 2GB memory limit
    /// let mut bridge = WASMNeuralBridge::new(Some(25), Some(2048))?;
    /// 
    /// // Create a classifier network
    /// bridge.create_network(
    ///     "mnist_classifier",
    ///     &[784, 256, 128, 10],
    ///     "relu,relu,sigmoid"
    /// )?;
    /// 
    /// // Spawn multiple agents for parallel processing
    /// let mut agents = Vec::new();
    /// for i in 0..5 {
    ///     let agent_id = bridge.spawn_agent("mnist_classifier", None)?;
    ///     agents.push(agent_id);
    /// }
    /// 
    /// // Run inference on multiple agents concurrently
    /// for agent_id in &agents {
    ///     let input = vec![0.5; 784]; // Normalized pixel data
    ///     let output = bridge.run_inference(agent_id, &input)?;
    ///     println!("Agent {} output: {:?}", agent_id, output);
    /// }
    /// 
    /// // Monitor performance
    /// let metrics = bridge.get_performance_metrics();
    /// println!("Performance Summary:");
    /// println!("  Active agents: {}", metrics.active_agents);
    /// println!("  Avg inference time: {:.2}ms", metrics.avg_inference_time_ms);
    /// println!("  Memory usage: {:.2}MB", metrics.memory_usage_mb);
    /// println!("  All targets met: {:?}", metrics.performance_targets);
    /// 
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn _basic_usage_example() {}
    
    /// # Performance Optimization Example
    /// 
    /// ```rust,no_run
    /// use ruv_fann_neural_bridge::{NeuralOptimizer, OptimizationConfig};
    /// 
    /// // Create optimizer with custom configuration
    /// let config = OptimizationConfig {
    ///     target_inference_latency_ms: 50.0, // Aggressive target
    ///     target_memory_per_agent_mb: 25.0,  // Aggressive memory target
    ///     enable_quantization: true,
    ///     enable_pruning: true,
    ///     ..Default::default()
    /// };
    /// 
    /// let mut optimizer = NeuralOptimizer::with_config(config);
    /// 
    /// // Auto-optimize a network
    /// let optimization_results = optimizer.auto_optimize(
    ///     "mnist_classifier",
    ///     "784x256x128x10",
    ///     784
    /// )?;
    /// 
    /// for result in optimization_results {
    ///     println!("Applied optimization: {}", result.strategy_name);
    ///     println!("  Latency improvement: {:.1}%", 
    ///              result.improvement_metrics.latency_improvement_percent);
    ///     println!("  Memory reduction: {:.1}%", 
    ///              result.improvement_metrics.memory_reduction_percent);
    /// }
    /// 
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn _optimization_example() {}
    
    /// # Performance Monitoring Example
    /// 
    /// ```rust,no_run
    /// use ruv_fann_neural_bridge::PerformanceMonitor;
    /// 
    /// let mut monitor = PerformanceMonitor::new();
    /// 
    /// // Record metrics during operation
    /// monitor.record_agent_spawn("agent_1", 45.0); // 45ms spawn time
    /// monitor.record_inference("agent_1", "network_1", 75.0, 784, 10, true, 1024);
    /// 
    /// // Get current performance snapshot
    /// let snapshot = monitor.get_current_metrics();
    /// 
    /// // Check for alerts
    /// let alerts = monitor.get_active_alerts();
    /// for alert in alerts {
    ///     match alert.severity {
    ///         AlertSeverity::Critical => println!("🚨 CRITICAL: {}", alert.message),
    ///         AlertSeverity::Warning => println!("⚠️  WARNING: {}", alert.message),
    ///         _ => println!("ℹ️  INFO: {}", alert.message),
    ///     }
    /// }
    /// 
    /// // Generate performance report
    /// let report = monitor.generate_report(ReportFormat::Text);
    /// println!("{}", report);
    /// ```
    fn _monitoring_example() {}
}