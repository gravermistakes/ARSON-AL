// Neural Network Optimizer for ruv-FANN WASM Integration
// Focuses on inference speed optimization and memory efficiency

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

/// Neural network optimization engine
pub struct NeuralOptimizer {
    optimization_cache: HashMap<String, OptimizationResult>,
    performance_profiles: Vec<PerformanceProfile>,
    optimization_strategies: Vec<OptimizationStrategy>,
    current_config: OptimizationConfig,
    metrics_collector: Arc<Mutex<OptimizationMetrics>>,
}

/// Configuration for neural network optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    pub target_inference_latency_ms: f64,    // Target: <100ms
    pub target_memory_per_agent_mb: f64,     // Target: <50MB
    pub target_accuracy_threshold: f64,      // Minimum accuracy to maintain
    pub enable_quantization: bool,           // Enable weight quantization
    pub enable_pruning: bool,               // Enable network pruning
    pub enable_knowledge_distillation: bool, // Enable model compression
    pub simd_optimization_level: SIMDLevel, // SIMD optimization aggressiveness
    pub memory_optimization_level: MemoryOptLevel, // Memory optimization level
    pub batch_inference_enabled: bool,      // Enable batch processing
    pub adaptive_optimization: bool,        // Enable runtime adaptation
}

/// SIMD optimization levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SIMDLevel {
    Disabled,    // No SIMD optimization
    Conservative, // Safe SIMD operations only
    Aggressive,  // Maximum SIMD utilization
    Adaptive,    // Runtime-adaptive SIMD usage
}

/// Memory optimization levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryOptLevel {
    Minimal,     // Basic memory management
    Balanced,    // Balance memory and performance
    Aggressive,  // Maximum memory efficiency
    Adaptive,    // Runtime-adaptive memory management
}

/// Optimization strategy definition
#[derive(Debug, Clone)]
pub struct OptimizationStrategy {
    pub name: String,
    pub description: String,
    pub target_metric: OptimizationTarget,
    pub implementation: StrategyImplementation,
    pub expected_improvement: f64,
    pub risk_level: RiskLevel,
    pub prerequisites: Vec<String>,
}

/// Optimization targets
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationTarget {
    InferenceLatency,
    MemoryUsage,
    Accuracy,
    Throughput,
    EnergyEfficiency,
    ModelSize,
}

/// Risk levels for optimization strategies
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,     // Safe optimizations with minimal impact
    Medium,  // Moderate risk with good expected benefit
    High,    // High risk but potentially high reward
    Critical, // Experimental optimizations
}

/// Strategy implementation details
#[derive(Debug, Clone)]
pub enum StrategyImplementation {
    WeightQuantization {
        bits: u8,
        method: QuantizationMethod,
    },
    NetworkPruning {
        sparsity_target: f64,
        pruning_method: PruningMethod,
    },
    KnowledgeDistillation {
        teacher_model: String,
        temperature: f64,
    },
    LayerFusion {
        fusion_patterns: Vec<String>,
    },
    MemoryPooling {
        pool_size: usize,
        allocation_strategy: String,
    },
    SIMDVectorization {
        operations: Vec<String>,
        vector_width: usize,
    },
    BatchOptimization {
        optimal_batch_size: usize,
        padding_strategy: String,
    },
    CacheOptimization {
        cache_size: usize,
        eviction_policy: String,
    },
}

/// Quantization methods
#[derive(Debug, Clone)]
pub enum QuantizationMethod {
    Uniform,
    NonUniform,
    DynamicRange,
    PostTrainingQuantization,
    QuantizationAwareTraining,
}

/// Pruning methods
#[derive(Debug, Clone)]
pub enum PruningMethod {
    MagnitudeBased,
    GradientBased,
    StructuredPruning,
    UnstructuredPruning,
    Progressive,
}

/// Performance profiling data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    pub profile_id: String,
    pub network_architecture: String,
    pub input_size: usize,
    pub inference_time_ms: f64,
    pub memory_usage_mb: f64,
    pub accuracy_score: f64,
    pub throughput_per_sec: f64,
    pub optimization_applied: Vec<String>,
    pub simd_utilization: f64,
    pub cache_hit_rate: f64,
    pub timestamp: u64,
}

/// Optimization result tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub optimization_id: String,
    pub strategy_name: String,
    pub baseline_performance: PerformanceProfile,
    pub optimized_performance: PerformanceProfile,
    pub improvement_metrics: ImprovementMetrics,
    pub optimization_time_ms: f64,
    pub success: bool,
    pub error_message: Option<String>,
    pub rollback_available: bool,
}

/// Detailed improvement metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementMetrics {
    pub latency_improvement_percent: f64,
    pub memory_reduction_percent: f64,
    pub accuracy_change_percent: f64,
    pub throughput_improvement_percent: f64,
    pub model_size_reduction_percent: f64,
    pub overall_score: f64,
}

/// Optimization metrics collection
#[derive(Debug, Default)]
pub struct OptimizationMetrics {
    pub total_optimizations: u32,
    pub successful_optimizations: u32,
    pub failed_optimizations: u32,
    pub total_improvement_time: f64,
    pub average_latency_improvement: f64,
    pub average_memory_reduction: f64,
    pub optimization_history: Vec<OptimizationResult>,
}

impl NeuralOptimizer {
    /// Create a new neural optimizer with default configuration
    pub fn new() -> Self {
        Self {
            optimization_cache: HashMap::new(),
            performance_profiles: Vec::new(),
            optimization_strategies: Self::initialize_strategies(),
            current_config: OptimizationConfig::default(),
            metrics_collector: Arc::new(Mutex::new(OptimizationMetrics::default())),
        }
    }
    
    /// Create optimizer with custom configuration
    pub fn with_config(config: OptimizationConfig) -> Self {
        Self {
            current_config: config,
            ..Self::new()
        }
    }
    
    /// Initialize available optimization strategies
    fn initialize_strategies() -> Vec<OptimizationStrategy> {
        vec![
            // Low-risk strategies
            OptimizationStrategy {
                name: "SIMD Vectorization".to_string(),
                description: "Enable SIMD instructions for matrix operations".to_string(),
                target_metric: OptimizationTarget::InferenceLatency,
                implementation: StrategyImplementation::SIMDVectorization {
                    operations: vec!["matrix_mul".to_string(), "activation".to_string()],
                    vector_width: 128,
                },
                expected_improvement: 25.0, // 25% latency improvement
                risk_level: RiskLevel::Low,
                prerequisites: vec!["simd_support".to_string()],
            },
            
            OptimizationStrategy {
                name: "Memory Pooling".to_string(),
                description: "Implement memory pooling for temporary allocations".to_string(),
                target_metric: OptimizationTarget::MemoryUsage,
                implementation: StrategyImplementation::MemoryPooling {
                    pool_size: 16 * 1024 * 1024, // 16MB pool
                    allocation_strategy: "best_fit".to_string(),
                },
                expected_improvement: 15.0, // 15% memory reduction
                risk_level: RiskLevel::Low,
                prerequisites: vec![],
            },
            
            OptimizationStrategy {
                name: "Batch Processing".to_string(),
                description: "Optimize for batch inference processing".to_string(),
                target_metric: OptimizationTarget::Throughput,
                implementation: StrategyImplementation::BatchOptimization {
                    optimal_batch_size: 8,
                    padding_strategy: "zero_pad".to_string(),
                },
                expected_improvement: 40.0, // 40% throughput improvement
                risk_level: RiskLevel::Low,
                prerequisites: vec![],
            },
            
            // Medium-risk strategies
            OptimizationStrategy {
                name: "Weight Quantization INT8".to_string(),
                description: "Quantize weights to 8-bit integers".to_string(),
                target_metric: OptimizationTarget::MemoryUsage,
                implementation: StrategyImplementation::WeightQuantization {
                    bits: 8,
                    method: QuantizationMethod::PostTrainingQuantization,
                },
                expected_improvement: 50.0, // 50% memory reduction
                risk_level: RiskLevel::Medium,
                prerequisites: vec!["quantization_calibration".to_string()],
            },
            
            OptimizationStrategy {
                name: "Magnitude-based Pruning".to_string(),
                description: "Remove low-magnitude weights to reduce model size".to_string(),
                target_metric: OptimizationTarget::ModelSize,
                implementation: StrategyImplementation::NetworkPruning {
                    sparsity_target: 0.3, // 30% sparsity
                    pruning_method: PruningMethod::MagnitudeBased,
                },
                expected_improvement: 30.0, // 30% model size reduction
                risk_level: RiskLevel::Medium,
                prerequisites: vec!["accuracy_baseline".to_string()],
            },
            
            // High-risk strategies
            OptimizationStrategy {
                name: "Aggressive Quantization INT4".to_string(),
                description: "Quantize weights to 4-bit integers".to_string(),
                target_metric: OptimizationTarget::MemoryUsage,
                implementation: StrategyImplementation::WeightQuantization {
                    bits: 4,
                    method: QuantizationMethod::QuantizationAwareTraining,
                },
                expected_improvement: 75.0, // 75% memory reduction
                risk_level: RiskLevel::High,
                prerequisites: vec!["retraining_capability".to_string()],
            },
            
            OptimizationStrategy {
                name: "Knowledge Distillation".to_string(),
                description: "Compress model using teacher-student distillation".to_string(),
                target_metric: OptimizationTarget::ModelSize,
                implementation: StrategyImplementation::KnowledgeDistillation {
                    teacher_model: "full_model".to_string(),
                    temperature: 4.0,
                },
                expected_improvement: 60.0, // 60% model size reduction
                risk_level: RiskLevel::High,
                prerequisites: vec!["teacher_model".to_string(), "training_data".to_string()],
            },
        ]
    }
    
    /// Profile network performance before optimization
    pub fn profile_network(&mut self, 
                          network_id: &str, 
                          network_arch: &str,
                          input_size: usize) -> Result<PerformanceProfile, OptimizationError> {
        let start_time = std::time::Instant::now();
        
        // Simulate performance profiling
        // In real implementation, this would run actual benchmarks
        let profile = PerformanceProfile {
            profile_id: format!("profile_{}_{}", network_id, chrono::Utc::now().timestamp()),
            network_architecture: network_arch.to_string(),
            input_size,
            inference_time_ms: self.simulate_inference_time(input_size),
            memory_usage_mb: self.estimate_memory_usage(network_arch, input_size),
            accuracy_score: 0.92, // Simulated baseline accuracy
            throughput_per_sec: 0.0, // Will be calculated
            optimization_applied: vec![],
            simd_utilization: 0.0, // No optimization yet
            cache_hit_rate: 0.0,
            timestamp: chrono::Utc::now().timestamp() as u64,
        };
        
        // Calculate throughput
        let mut updated_profile = profile.clone();
        updated_profile.throughput_per_sec = 1000.0 / profile.inference_time_ms;
        
        self.performance_profiles.push(updated_profile.clone());
        
        println!("📊 Network profiling completed in {:.2}ms", start_time.elapsed().as_millis());
        println!("   Baseline inference time: {:.2}ms", updated_profile.inference_time_ms);
        println!("   Baseline memory usage: {:.2}MB", updated_profile.memory_usage_mb);
        
        Ok(updated_profile)
    }
    
    /// Recommend optimization strategies based on profile
    pub fn recommend_optimizations(&self, 
                                 profile: &PerformanceProfile) -> Vec<OptimizationRecommendation> {
        let mut recommendations = Vec::new();
        
        // Analyze performance bottlenecks
        let latency_bottleneck = profile.inference_time_ms > self.current_config.target_inference_latency_ms;
        let memory_bottleneck = profile.memory_usage_mb > self.current_config.target_memory_per_agent_mb;
        let accuracy_headroom = profile.accuracy_score > self.current_config.target_accuracy_threshold + 0.05;
        
        for strategy in &self.optimization_strategies {
            let mut recommendation_score = 0.0;
            let mut applicable = true;
            let mut reasons = Vec::new();
            
            // Check if strategy addresses current bottlenecks
            match strategy.target_metric {
                OptimizationTarget::InferenceLatency if latency_bottleneck => {
                    recommendation_score += 0.4;
                    reasons.push("Addresses inference latency bottleneck".to_string());
                },
                OptimizationTarget::MemoryUsage if memory_bottleneck => {
                    recommendation_score += 0.4;
                    reasons.push("Addresses memory usage bottleneck".to_string());
                },
                OptimizationTarget::ModelSize if accuracy_headroom => {
                    recommendation_score += 0.3;
                    reasons.push("Accuracy headroom allows model compression".to_string());
                },
                _ => {}
            }
            
            // Adjust score based on risk level and configuration
            match strategy.risk_level {
                RiskLevel::Low => recommendation_score += 0.2,
                RiskLevel::Medium => recommendation_score += 0.1,
                RiskLevel::High => {
                    if !accuracy_headroom {
                        applicable = false;
                        reasons.push("Insufficient accuracy headroom for high-risk optimization".to_string());
                    }
                },
                RiskLevel::Critical => {
                    applicable = false;
                    reasons.push("Critical risk level not recommended".to_string());
                }
            }
            
            // Check prerequisites
            if strategy.prerequisites.contains(&"simd_support".to_string()) && 
               !cfg!(target_feature = "simd128") {
                applicable = false;
                reasons.push("SIMD support not available".to_string());
            }
            
            if applicable && recommendation_score > 0.1 {
                recommendations.push(OptimizationRecommendation {
                    strategy: strategy.clone(),
                    recommendation_score,
                    priority: if recommendation_score > 0.5 { Priority::High } 
                             else if recommendation_score > 0.3 { Priority::Medium } 
                             else { Priority::Low },
                    estimated_improvement: strategy.expected_improvement,
                    reasons,
                    prerequisites_met: true,
                });
            }
        }
        
        // Sort by recommendation score
        recommendations.sort_by(|a, b| b.recommendation_score.partial_cmp(&a.recommendation_score).unwrap());
        
        recommendations
    }
    
    /// Apply optimization strategy to network
    pub fn apply_optimization(&mut self,
                            network_id: &str,
                            strategy: &OptimizationStrategy,
                            baseline_profile: &PerformanceProfile) -> Result<OptimizationResult, OptimizationError> {
        let start_time = std::time::Instant::now();
        let optimization_id = format!("opt_{}_{}", network_id, chrono::Utc::now().timestamp());
        
        println!("🔧 Applying optimization: {} to network {}", strategy.name, network_id);
        
        // Simulate optimization process based on strategy
        let result = match &strategy.implementation {
            StrategyImplementation::SIMDVectorization { operations, vector_width } => {
                self.apply_simd_optimization(network_id, operations, *vector_width)
            },
            StrategyImplementation::MemoryPooling { pool_size, allocation_strategy } => {
                self.apply_memory_pooling(network_id, *pool_size, allocation_strategy)
            },
            StrategyImplementation::WeightQuantization { bits, method } => {
                self.apply_weight_quantization(network_id, *bits, method)
            },
            StrategyImplementation::NetworkPruning { sparsity_target, pruning_method } => {
                self.apply_network_pruning(network_id, *sparsity_target, pruning_method)
            },
            StrategyImplementation::BatchOptimization { optimal_batch_size, padding_strategy } => {
                self.apply_batch_optimization(network_id, *optimal_batch_size, padding_strategy)
            },
            _ => {
                return Err(OptimizationError::UnsupportedStrategy(strategy.name.clone()));
            }
        };
        
        let optimization_time = start_time.elapsed().as_millis() as f64;
        
        match result {
            Ok(optimized_profile) => {
                let improvement_metrics = self.calculate_improvement_metrics(baseline_profile, &optimized_profile);
                
                let optimization_result = OptimizationResult {
                    optimization_id: optimization_id.clone(),
                    strategy_name: strategy.name.clone(),
                    baseline_performance: baseline_profile.clone(),
                    optimized_performance: optimized_profile,
                    improvement_metrics,
                    optimization_time_ms: optimization_time,
                    success: true,
                    error_message: None,
                    rollback_available: true,
                };
                
                // Cache the result
                self.optimization_cache.insert(optimization_id.clone(), optimization_result.clone());
                
                // Update metrics
                if let Ok(mut metrics) = self.metrics_collector.lock() {
                    metrics.total_optimizations += 1;
                    metrics.successful_optimizations += 1;
                    metrics.total_improvement_time += optimization_time;
                    metrics.average_latency_improvement = 
                        (metrics.average_latency_improvement * (metrics.successful_optimizations - 1) as f64 + 
                         optimization_result.improvement_metrics.latency_improvement_percent) / 
                        metrics.successful_optimizations as f64;
                    metrics.optimization_history.push(optimization_result.clone());
                }
                
                println!("✅ Optimization completed successfully in {:.2}ms", optimization_time);
                println!("   Latency improvement: {:.1}%", optimization_result.improvement_metrics.latency_improvement_percent);
                println!("   Memory reduction: {:.1}%", optimization_result.improvement_metrics.memory_reduction_percent);
                
                Ok(optimization_result)
            },
            Err(error) => {
                let optimization_result = OptimizationResult {
                    optimization_id,
                    strategy_name: strategy.name.clone(),
                    baseline_performance: baseline_profile.clone(),
                    optimized_performance: baseline_profile.clone(), // No change
                    improvement_metrics: ImprovementMetrics::default(),
                    optimization_time_ms: optimization_time,
                    success: false,
                    error_message: Some(error.to_string()),
                    rollback_available: false,
                };
                
                // Update error metrics
                if let Ok(mut metrics) = self.metrics_collector.lock() {
                    metrics.total_optimizations += 1;
                    metrics.failed_optimizations += 1;
                }
                
                println!("❌ Optimization failed: {}", error);
                
                Ok(optimization_result)
            }
        }
    }
    
    /// Get optimization metrics summary
    pub fn get_optimization_metrics(&self) -> OptimizationMetrics {
        self.metrics_collector.lock().unwrap().clone()
    }
    
    /// Auto-optimize network based on current performance
    pub fn auto_optimize(&mut self, 
                        network_id: &str, 
                        network_arch: &str,
                        input_size: usize) -> Result<Vec<OptimizationResult>, OptimizationError> {
        println!("🤖 Starting auto-optimization for network {}", network_id);
        
        // Profile current performance
        let baseline_profile = self.profile_network(network_id, network_arch, input_size)?;
        
        // Get recommendations
        let recommendations = self.recommend_optimizations(&baseline_profile);
        
        if recommendations.is_empty() {
            println!("ℹ️ No optimizations recommended for current performance profile");
            return Ok(vec![]);
        }
        
        let mut optimization_results = Vec::new();
        let mut current_profile = baseline_profile.clone();
        
        // Apply optimizations in order of priority
        for recommendation in recommendations.iter().take(3) { // Limit to top 3 recommendations
            if recommendation.priority == Priority::High || 
               (recommendation.priority == Priority::Medium && self.current_config.adaptive_optimization) {
                
                match self.apply_optimization(network_id, &recommendation.strategy, &current_profile) {
                    Ok(result) => {
                        if result.success && result.improvement_metrics.overall_score > 0.0 {
                            current_profile = result.optimized_performance.clone();
                            optimization_results.push(result);
                        }
                    },
                    Err(e) => {
                        println!("⚠️ Skipping optimization {} due to error: {}", recommendation.strategy.name, e);
                    }
                }
            }
        }
        
        println!("✅ Auto-optimization completed. Applied {} optimizations", optimization_results.len());
        
        Ok(optimization_results)
    }
}

// Private implementation methods
impl NeuralOptimizer {
    fn simulate_inference_time(&self, input_size: usize) -> f64 {
        // Simulate inference time based on input size and current optimizations
        let base_time = (input_size as f64).sqrt() * 2.0; // Base complexity
        
        // Add random variance
        base_time + (fastrand::f64() * 20.0 - 10.0)
    }
    
    fn estimate_memory_usage(&self, network_arch: &str, input_size: usize) -> f64 {
        // Estimate memory usage based on architecture
        let base_memory = input_size as f64 * 0.001; // 1KB per input unit
        let arch_multiplier = if network_arch.contains("large") { 2.0 } else { 1.0 };
        
        base_memory * arch_multiplier + 5.0 // 5MB base overhead
    }
    
    fn apply_simd_optimization(&self, 
                              _network_id: &str,
                              operations: &[String],
                              _vector_width: usize) -> Result<PerformanceProfile, OptimizationError> {
        // Simulate SIMD optimization effects
        let improvement_factor = 0.75; // 25% improvement
        
        // Create optimized profile (simulation)
        let mut profile = self.performance_profiles.last().unwrap().clone();
        profile.inference_time_ms *= improvement_factor;
        profile.simd_utilization = 85.0; // High SIMD utilization
        profile.optimization_applied.push("SIMD Vectorization".to_string());
        
        Ok(profile)
    }
    
    fn apply_memory_pooling(&self,
                           _network_id: &str,
                           _pool_size: usize,
                           _allocation_strategy: &str) -> Result<PerformanceProfile, OptimizationError> {
        // Simulate memory pooling effects
        let mut profile = self.performance_profiles.last().unwrap().clone();
        profile.memory_usage_mb *= 0.85; // 15% memory reduction
        profile.optimization_applied.push("Memory Pooling".to_string());
        
        Ok(profile)
    }
    
    fn apply_weight_quantization(&self,
                                _network_id: &str,
                                bits: u8,
                                _method: &QuantizationMethod) -> Result<PerformanceProfile, OptimizationError> {
        // Simulate quantization effects
        let memory_reduction = match bits {
            8 => 0.5,  // 50% reduction (32-bit to 8-bit)
            4 => 0.25, // 75% reduction
            _ => return Err(OptimizationError::InvalidParameter("Unsupported quantization bits".to_string())),
        };
        
        let accuracy_loss = match bits {
            8 => 0.02,  // 2% accuracy loss
            4 => 0.05,  // 5% accuracy loss
            _ => 0.0,
        };
        
        let mut profile = self.performance_profiles.last().unwrap().clone();
        profile.memory_usage_mb *= memory_reduction;
        profile.accuracy_score -= accuracy_loss;
        profile.inference_time_ms *= 0.9; // Slight inference improvement due to cache efficiency
        profile.optimization_applied.push(format!("Weight Quantization ({}bit)", bits));
        
        Ok(profile)
    }
    
    fn apply_network_pruning(&self,
                            _network_id: &str,
                            sparsity_target: f64,
                            _pruning_method: &PruningMethod) -> Result<PerformanceProfile, OptimizationError> {
        if sparsity_target > 0.8 {
            return Err(OptimizationError::InvalidParameter("Sparsity target too high".to_string()));
        }
        
        // Simulate pruning effects
        let mut profile = self.performance_profiles.last().unwrap().clone();
        profile.memory_usage_mb *= (1.0 - sparsity_target);
        profile.inference_time_ms *= (1.0 - sparsity_target * 0.5); // Partial speedup
        profile.accuracy_score -= sparsity_target * 0.1; // Proportional accuracy loss
        profile.optimization_applied.push(format!("Network Pruning ({:.1}% sparse)", sparsity_target * 100.0));
        
        Ok(profile)
    }
    
    fn apply_batch_optimization(&self,
                               _network_id: &str,
                               _optimal_batch_size: usize,
                               _padding_strategy: &str) -> Result<PerformanceProfile, OptimizationError> {
        // Simulate batch optimization effects
        let mut profile = self.performance_profiles.last().unwrap().clone();
        profile.throughput_per_sec *= 1.4; // 40% throughput improvement
        profile.inference_time_ms *= 0.8; // 20% latency improvement for individual inferences
        profile.optimization_applied.push("Batch Optimization".to_string());
        
        Ok(profile)
    }
    
    fn calculate_improvement_metrics(&self, 
                                   baseline: &PerformanceProfile, 
                                   optimized: &PerformanceProfile) -> ImprovementMetrics {
        let latency_improvement = (baseline.inference_time_ms - optimized.inference_time_ms) / baseline.inference_time_ms * 100.0;
        let memory_reduction = (baseline.memory_usage_mb - optimized.memory_usage_mb) / baseline.memory_usage_mb * 100.0;
        let accuracy_change = (optimized.accuracy_score - baseline.accuracy_score) / baseline.accuracy_score * 100.0;
        let throughput_improvement = (optimized.throughput_per_sec - baseline.throughput_per_sec) / baseline.throughput_per_sec * 100.0;
        
        // Calculate overall score (weighted average)
        let overall_score = (latency_improvement * 0.3 + 
                           memory_reduction * 0.3 + 
                           accuracy_change * 0.2 + 
                           throughput_improvement * 0.2).max(0.0);
        
        ImprovementMetrics {
            latency_improvement_percent: latency_improvement,
            memory_reduction_percent: memory_reduction,
            accuracy_change_percent: accuracy_change,
            throughput_improvement_percent: throughput_improvement,
            model_size_reduction_percent: memory_reduction, // Simplified
            overall_score,
        }
    }
}

// Supporting structures
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    pub strategy: OptimizationStrategy,
    pub recommendation_score: f64,
    pub priority: Priority,
    pub estimated_improvement: f64,
    pub reasons: Vec<String>,
    pub prerequisites_met: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug)]
pub enum OptimizationError {
    NetworkNotFound(String),
    UnsupportedStrategy(String),
    InvalidParameter(String),
    InsufficientAccuracy(f64),
    OptimizationFailed(String),
    PrerequisiteNotMet(String),
}

impl std::fmt::Display for OptimizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptimizationError::NetworkNotFound(id) => write!(f, "Network not found: {}", id),
            OptimizationError::UnsupportedStrategy(name) => write!(f, "Unsupported strategy: {}", name),
            OptimizationError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            OptimizationError::InsufficientAccuracy(threshold) => write!(f, "Insufficient accuracy: below {}", threshold),
            OptimizationError::OptimizationFailed(msg) => write!(f, "Optimization failed: {}", msg),
            OptimizationError::PrerequisiteNotMet(prereq) => write!(f, "Prerequisite not met: {}", prereq),
        }
    }
}

impl std::error::Error for OptimizationError {}

// Default implementations
impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            target_inference_latency_ms: 100.0,
            target_memory_per_agent_mb: 50.0,
            target_accuracy_threshold: 0.85,
            enable_quantization: true,
            enable_pruning: true,
            enable_knowledge_distillation: false,
            simd_optimization_level: SIMDLevel::Aggressive,
            memory_optimization_level: MemoryOptLevel::Balanced,
            batch_inference_enabled: true,
            adaptive_optimization: true,
        }
    }
}

impl Default for ImprovementMetrics {
    fn default() -> Self {
        Self {
            latency_improvement_percent: 0.0,
            memory_reduction_percent: 0.0,
            accuracy_change_percent: 0.0,
            throughput_improvement_percent: 0.0,
            model_size_reduction_percent: 0.0,
            overall_score: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_optimizer_creation() {
        let optimizer = NeuralOptimizer::new();
        assert!(!optimizer.optimization_strategies.is_empty());
        assert_eq!(optimizer.current_config.target_inference_latency_ms, 100.0);
    }
    
    #[test]
    fn test_performance_profiling() {
        let mut optimizer = NeuralOptimizer::new();
        let profile = optimizer.profile_network("test_net", "simple", 100).unwrap();
        
        assert_eq!(profile.network_architecture, "simple");
        assert_eq!(profile.input_size, 100);
        assert!(profile.inference_time_ms > 0.0);
        assert!(profile.memory_usage_mb > 0.0);
    }
    
    #[test]
    fn test_optimization_recommendations() {
        let optimizer = NeuralOptimizer::new();
        let profile = PerformanceProfile {
            profile_id: "test".to_string(),
            network_architecture: "test".to_string(),
            input_size: 100,
            inference_time_ms: 150.0, // Above threshold
            memory_usage_mb: 60.0,    // Above threshold
            accuracy_score: 0.95,     // High accuracy
            throughput_per_sec: 6.67,
            optimization_applied: vec![],
            simd_utilization: 0.0,
            cache_hit_rate: 0.0,
            timestamp: 0,
        };
        
        let recommendations = optimizer.recommend_optimizations(&profile);
        assert!(!recommendations.is_empty());
        
        // Should recommend latency and memory optimizations
        let has_latency_opt = recommendations.iter().any(|r| r.strategy.target_metric == OptimizationTarget::InferenceLatency);
        let has_memory_opt = recommendations.iter().any(|r| r.strategy.target_metric == OptimizationTarget::MemoryUsage);
        
        assert!(has_latency_opt || has_memory_opt);
    }
}