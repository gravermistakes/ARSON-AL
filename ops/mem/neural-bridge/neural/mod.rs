// Neural network optimization module

pub mod optimizer;

// Re-export main types
pub use optimizer::{
    NeuralOptimizer, OptimizationConfig, OptimizationStrategy,
    OptimizationResult, PerformanceProfile
};