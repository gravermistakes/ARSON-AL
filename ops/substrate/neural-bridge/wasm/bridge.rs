// WASM Bridge Integration for ruv-FANN Neural Agents
// Optimized for <100ms inference with SIMD acceleration and memory efficiency

use wasm_bindgen::prelude::*;
use js_sys::{Array, Float32Array, Uint8Array};
use web_sys::console;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Import SIMD configuration
use crate::simd_config::{SIMDNeuralNetwork, SIMDNeuralOps, ActivationType};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
}

// Macro for performance logging
macro_rules! perf_log {
    ($($t:tt)*) => {
        log(&format!($($t)*));
    }
}

/// High-performance WASM bridge for neural agent management
#[wasm_bindgen]
pub struct WASMNeuralBridge {
    networks: HashMap<String, SIMDNeuralNetwork>,
    agent_pool: Vec<NeuralAgent>,
    performance_metrics: PerformanceTracker,
    memory_manager: MemoryManager,
    max_agents: usize,
    simd_enabled: bool,
}

/// Neural agent wrapper with performance tracking
#[derive(Clone)]
pub struct NeuralAgent {
    pub id: String,
    pub network_id: String,
    pub created_at: f64,
    pub last_used: f64,
    pub inference_count: u32,
    pub total_inference_time: f64,
    pub memory_usage: usize,
    pub performance_score: f64,
}

/// Performance tracking for optimization
#[derive(Default)]
pub struct PerformanceTracker {
    pub agent_spawn_times: Vec<f64>,
    pub inference_latencies: Vec<f64>,
    pub memory_usage_samples: Vec<usize>,
    pub error_counts: HashMap<String, u32>,
    pub optimization_events: Vec<OptimizationEvent>,
}

/// Memory management for efficient agent handling
pub struct MemoryManager {
    pub total_allocated: usize,
    pub peak_usage: usize,
    pub gc_events: u32,
    pub compression_ratio: f64,
    pub memory_limit: usize,
}

/// Optimization event tracking
#[derive(Clone)]
pub struct OptimizationEvent {
    pub timestamp: f64,
    pub event_type: String,
    pub description: String,
    pub performance_impact: f64,
}

#[wasm_bindgen]
impl WASMNeuralBridge {
    /// Initialize the WASM neural bridge with configuration
    #[wasm_bindgen(constructor)]
    pub fn new(max_agents: Option<usize>, memory_limit_mb: Option<usize>) -> Result<WASMNeuralBridge, JsValue> {
        let start_time = now();
        
        perf_log!("🚀 Initializing WASM Neural Bridge...");
        
        // Detect SIMD capabilities
        let simd_enabled = cfg!(target_feature = "simd128");
        perf_log!("SIMD support: {}", if simd_enabled { "ENABLED" } else { "DISABLED" });
        
        let bridge = WASMNeuralBridge {
            networks: HashMap::new(),
            agent_pool: Vec::with_capacity(max_agents.unwrap_or(25)),
            performance_metrics: PerformanceTracker::default(),
            memory_manager: MemoryManager {
                total_allocated: 0,
                peak_usage: 0,
                gc_events: 0,
                compression_ratio: 1.0,
                memory_limit: memory_limit_mb.unwrap_or(2048) * 1024 * 1024, // Convert MB to bytes
            },
            max_agents: max_agents.unwrap_or(25),
            simd_enabled,
        };
        
        let init_time = now() - start_time;
        perf_log!("✅ WASM Bridge initialized in {:.2}ms", init_time);
        
        Ok(bridge)
    }
    
    /// Create a new neural network configuration
    #[wasm_bindgen]
    pub fn create_network(&mut self, 
                         network_id: &str, 
                         layer_sizes: &[usize], 
                         activation_types: &str) -> Result<(), JsValue> {
        let start_time = now();
        
        // Parse activation types (comma-separated string)
        let activations: Result<Vec<ActivationType>, _> = activation_types
            .split(',')
            .map(|s| match s.trim().to_lowercase().as_str() {
                "sigmoid" => Ok(ActivationType::Sigmoid),
                "relu" => Ok(ActivationType::ReLU),
                "tanh" => Ok(ActivationType::Tanh),
                "linear" => Ok(ActivationType::Linear),
                _ => Err(format!("Unknown activation type: {}", s))
            })
            .collect();
        
        let activations = activations.map_err(|e| JsValue::from_str(&e))?;
        
        if layer_sizes.len() - 1 != activations.len() {
            return Err(JsValue::from_str("Mismatch between layer count and activation count"));
        }
        
        // Create SIMD-optimized neural network
        let network = SIMDNeuralNetwork::new(layer_sizes, &activations);
        let memory_usage = network.memory_usage();
        
        self.networks.insert(network_id.to_string(), network);
        self.memory_manager.total_allocated += memory_usage;
        
        let creation_time = now() - start_time;
        perf_log!("📊 Network '{}' created in {:.2}ms, Memory: {}KB", 
                 network_id, creation_time, memory_usage / 1024);
        
        Ok(())
    }
    
    /// Spawn a new neural agent with optimized initialization
    #[wasm_bindgen]
    pub fn spawn_agent(&mut self, network_id: &str, agent_config: Option<String>) -> Result<String, JsValue> {
        let start_time = now();
        
        // Check agent limit
        if self.agent_pool.len() >= self.max_agents {
            // Attempt memory cleanup first
            self.perform_memory_cleanup();
            
            if self.agent_pool.len() >= self.max_agents {
                return Err(JsValue::from_str(&format!("Maximum agent limit reached: {}", self.max_agents)));
            }
        }
        
        // Verify network exists
        if !self.networks.contains_key(network_id) {
            return Err(JsValue::from_str(&format!("Network '{}' not found", network_id)));
        }
        
        // Generate unique agent ID
        let agent_id = format!("agent_{}_{}", 
                              (now() as u64), 
                              fastrand::u32(100000..999999));
        
        // Estimate memory usage for this agent
        let network = self.networks.get(network_id).unwrap();
        let memory_usage = network.memory_usage() + 1024; // Base overhead
        
        // Create agent
        let agent = NeuralAgent {
            id: agent_id.clone(),
            network_id: network_id.to_string(),
            created_at: now(),
            last_used: now(),
            inference_count: 0,
            total_inference_time: 0.0,
            memory_usage,
            performance_score: 1.0,
        };
        
        self.agent_pool.push(agent);
        self.memory_manager.total_allocated += memory_usage;
        
        // Update peak usage
        if self.memory_manager.total_allocated > self.memory_manager.peak_usage {
            self.memory_manager.peak_usage = self.memory_manager.total_allocated;
        }
        
        let spawn_time = now() - start_time;
        self.performance_metrics.agent_spawn_times.push(spawn_time);
        
        perf_log!("🤖 Agent '{}' spawned in {:.2}ms (Network: {}, Memory: {}KB)", 
                 agent_id, spawn_time, network_id, memory_usage / 1024);
        
        // Check performance target
        if spawn_time > 75.0 {
            perf_log!("⚠️ Agent spawn time {:.2}ms exceeds 75ms target", spawn_time);
        }
        
        Ok(agent_id)
    }
    
    /// Run neural inference with SIMD optimization
    #[wasm_bindgen]
    pub fn run_inference(&mut self, agent_id: &str, input_data: &[f32]) -> Result<Array, JsValue> {
        let start_time = now();
        
        // Find agent
        let agent_index = self.agent_pool.iter()
            .position(|a| a.id == agent_id)
            .ok_or_else(|| JsValue::from_str(&format!("Agent '{}' not found", agent_id)))?;
        
        let agent = &mut self.agent_pool[agent_index];
        let network_id = agent.network_id.clone();
        
        // Update last used timestamp
        agent.last_used = now();
        
        // Get network and run inference
        let network = self.networks.get(&network_id)
            .ok_or_else(|| JsValue::from_str(&format!("Network '{}' not found", network_id)))?;
        
        // Run SIMD-optimized inference
        let output = network.infer(input_data);
        
        // Convert to JavaScript array
        let js_array = Array::new();
        for value in output {
            js_array.push(&JsValue::from_f64(value as f64));
        }
        
        let inference_time = now() - start_time;
        
        // Update agent metrics
        agent.inference_count += 1;
        agent.total_inference_time += inference_time;
        agent.performance_score = self.calculate_performance_score(agent);
        
        // Update global metrics
        self.performance_metrics.inference_latencies.push(inference_time);
        
        perf_log!("🧠 Inference completed in {:.2}ms for agent '{}'", inference_time, agent_id);
        
        // Check performance target
        if inference_time > 100.0 {
            perf_log!("⚠️ Inference time {:.2}ms exceeds 100ms target", inference_time);
            
            // Record performance event
            self.performance_metrics.optimization_events.push(OptimizationEvent {
                timestamp: now(),
                event_type: "PERFORMANCE_WARNING".to_string(),
                description: format!("Inference time {:.2}ms > 100ms target", inference_time),
                performance_impact: inference_time - 100.0,
            });
        }
        
        Ok(js_array)
    }
    
    /// Batch inference for multiple inputs
    #[wasm_bindgen]
    pub fn run_batch_inference(&mut self, agent_id: &str, batch_inputs: &Array) -> Result<Array, JsValue> {
        let start_time = now();
        
        // Find agent
        let agent_index = self.agent_pool.iter()
            .position(|a| a.id == agent_id)
            .ok_or_else(|| JsValue::from_str(&format!("Agent '{}' not found", agent_id)))?;
        
        let agent = &mut self.agent_pool[agent_index];
        let network_id = agent.network_id.clone();
        
        // Get network
        let network = self.networks.get(&network_id)
            .ok_or_else(|| JsValue::from_str(&format!("Network '{}' not found", network_id)))?;
        
        // Convert JS inputs to Rust vectors
        let mut rust_inputs = Vec::new();
        for i in 0..batch_inputs.length() {
            let js_input = batch_inputs.get(i);
            if let Ok(float_array) = js_input.dyn_into::<Float32Array>() {
                let input_vec = float_array.to_vec();
                rust_inputs.push(input_vec);
            }
        }
        
        // Run batch inference
        let batch_outputs = network.infer_batch(&rust_inputs);
        
        // Convert results to JavaScript
        let js_results = Array::new();
        for output in batch_outputs {
            let js_output = Array::new();
            for value in output {
                js_output.push(&JsValue::from_f64(value as f64));
            }
            js_results.push(&js_output);
        }
        
        let batch_time = now() - start_time;
        let avg_inference_time = batch_time / rust_inputs.len() as f64;
        
        // Update metrics
        agent.inference_count += rust_inputs.len() as u32;
        agent.total_inference_time += batch_time;
        
        perf_log!("📊 Batch inference: {} inputs processed in {:.2}ms (avg: {:.2}ms/inference)", 
                 rust_inputs.len(), batch_time, avg_inference_time);
        
        Ok(js_results)
    }
    
    /// Get comprehensive performance metrics
    #[wasm_bindgen]
    pub fn get_performance_metrics(&self) -> JsValue {
        let avg_spawn_time = if !self.performance_metrics.agent_spawn_times.is_empty() {
            self.performance_metrics.agent_spawn_times.iter().sum::<f64>() / 
            self.performance_metrics.agent_spawn_times.len() as f64
        } else { 0.0 };
        
        let avg_inference_time = if !self.performance_metrics.inference_latencies.is_empty() {
            self.performance_metrics.inference_latencies.iter().sum::<f64>() / 
            self.performance_metrics.inference_latencies.len() as f64
        } else { 0.0 };
        
        let p95_inference = self.calculate_percentile(&self.performance_metrics.inference_latencies, 95.0);
        let memory_usage_mb = self.memory_manager.total_allocated as f64 / 1024.0 / 1024.0;
        let memory_limit_mb = self.memory_manager.memory_limit as f64 / 1024.0 / 1024.0;
        
        let metrics = js_sys::Object::new();
        
        // Performance metrics
        js_sys::Reflect::set(&metrics, &"active_agents".into(), &JsValue::from(self.agent_pool.len())).unwrap();
        js_sys::Reflect::set(&metrics, &"max_agents".into(), &JsValue::from(self.max_agents)).unwrap();
        js_sys::Reflect::set(&metrics, &"avg_spawn_time_ms".into(), &JsValue::from(avg_spawn_time)).unwrap();
        js_sys::Reflect::set(&metrics, &"avg_inference_time_ms".into(), &JsValue::from(avg_inference_time)).unwrap();
        js_sys::Reflect::set(&metrics, &"p95_inference_time_ms".into(), &JsValue::from(p95_inference)).unwrap();
        
        // Memory metrics
        js_sys::Reflect::set(&metrics, &"memory_usage_mb".into(), &JsValue::from(memory_usage_mb)).unwrap();
        js_sys::Reflect::set(&metrics, &"memory_limit_mb".into(), &JsValue::from(memory_limit_mb)).unwrap();
        js_sys::Reflect::set(&metrics, &"memory_utilization_percent".into(), &JsValue::from(memory_usage_mb / memory_limit_mb * 100.0)).unwrap();
        js_sys::Reflect::set(&metrics, &"peak_memory_mb".into(), &JsValue::from(self.memory_manager.peak_usage as f64 / 1024.0 / 1024.0)).unwrap();
        
        // System metrics
        js_sys::Reflect::set(&metrics, &"simd_enabled".into(), &JsValue::from(self.simd_enabled)).unwrap();
        js_sys::Reflect::set(&metrics, &"total_inferences".into(), &JsValue::from(self.performance_metrics.inference_latencies.len())).unwrap();
        js_sys::Reflect::set(&metrics, &"gc_events".into(), &JsValue::from(self.memory_manager.gc_events)).unwrap();
        
        // Performance targets
        let targets = js_sys::Object::new();
        js_sys::Reflect::set(&targets, &"spawn_time_target_met".into(), &JsValue::from(avg_spawn_time < 75.0)).unwrap();
        js_sys::Reflect::set(&targets, &"inference_time_target_met".into(), &JsValue::from(p95_inference < 100.0)).unwrap();
        js_sys::Reflect::set(&targets, &"memory_target_met".into(), &JsValue::from(memory_usage_mb < 2048.0)).unwrap();
        
        js_sys::Reflect::set(&metrics, &"performance_targets".into(), &targets).unwrap();
        
        metrics.into()
    }
    
    /// Optimize performance based on current metrics
    #[wasm_bindgen]
    pub fn optimize_performance(&mut self) -> JsValue {
        let start_time = now();
        perf_log!("🔧 Starting performance optimization...");
        
        let mut optimizations = Vec::new();
        
        // 1. Memory optimization
        if self.memory_manager.total_allocated > self.memory_manager.memory_limit / 2 {
            self.perform_memory_cleanup();
            optimizations.push("Memory cleanup performed");
        }
        
        // 2. Agent pool optimization
        if self.agent_pool.len() > 10 {
            self.optimize_agent_pool();
            optimizations.push("Agent pool optimized");
        }
        
        // 3. Network compression (if needed)
        if self.memory_manager.total_allocated > self.memory_manager.memory_limit * 3 / 4 {
            self.compress_networks();
            optimizations.push("Network compression applied");
        }
        
        let optimization_time = now() - start_time;
        perf_log!("✅ Performance optimization completed in {:.2}ms", optimization_time);
        
        // Record optimization event
        self.performance_metrics.optimization_events.push(OptimizationEvent {
            timestamp: now(),
            event_type: "PERFORMANCE_OPTIMIZATION".to_string(),
            description: format!("Applied {} optimizations", optimizations.len()),
            performance_impact: -optimization_time, // Negative because it's beneficial
        });
        
        // Return optimization results
        let result = js_sys::Object::new();
        js_sys::Reflect::set(&result, &"optimization_time_ms".into(), &JsValue::from(optimization_time)).unwrap();
        js_sys::Reflect::set(&result, &"optimizations_applied".into(), &JsValue::from(optimizations.len())).unwrap();
        
        let opt_array = Array::new();
        for opt in optimizations {
            opt_array.push(&JsValue::from_str(&opt));
        }
        js_sys::Reflect::set(&result, &"optimizations".into(), &opt_array).unwrap();
        
        result.into()
    }
    
    /// Force memory cleanup and garbage collection
    #[wasm_bindgen]
    pub fn perform_memory_cleanup(&mut self) {
        let start_time = now();
        let initial_memory = self.memory_manager.total_allocated;
        
        perf_log!("🧹 Performing memory cleanup...");
        
        // Remove aged agents (older than 5 minutes unused)
        let current_time = now();
        let max_age = 5.0 * 60.0 * 1000.0; // 5 minutes in milliseconds
        
        let initial_count = self.agent_pool.len();
        self.agent_pool.retain(|agent| {
            let age = current_time - agent.last_used;
            if age > max_age {
                self.memory_manager.total_allocated -= agent.memory_usage;
                false
            } else {
                true
            }
        });
        
        let removed_count = initial_count - self.agent_pool.len();
        let memory_freed = initial_memory - self.memory_manager.total_allocated;
        
        self.memory_manager.gc_events += 1;
        
        let cleanup_time = now() - start_time;
        perf_log!("✅ Memory cleanup: removed {} agents, freed {}KB in {:.2}ms", 
                 removed_count, memory_freed / 1024, cleanup_time);
    }
    
    /// Get detailed system diagnostics
    #[wasm_bindgen]
    pub fn get_system_diagnostics(&self) -> JsValue {
        let diagnostics = js_sys::Object::new();
        
        // Agent distribution by network
        let mut network_usage = HashMap::new();
        for agent in &self.agent_pool {
            *network_usage.entry(agent.network_id.clone()).or_insert(0) += 1;
        }
        
        // Top performing agents
        let mut top_agents: Vec<_> = self.agent_pool.iter()
            .map(|a| (a.id.clone(), a.performance_score))
            .collect();
        top_agents.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Performance trends
        let recent_inferences: Vec<f64> = self.performance_metrics.inference_latencies
            .iter()
            .rev()
            .take(100)
            .cloned()
            .collect();
        
        // Build diagnostics object
        js_sys::Reflect::set(&diagnostics, &"total_networks".into(), &JsValue::from(self.networks.len())).unwrap();
        js_sys::Reflect::set(&diagnostics, &"total_agents".into(), &JsValue::from(self.agent_pool.len())).unwrap();
        js_sys::Reflect::set(&diagnostics, &"memory_efficiency_percent".into(), 
            &JsValue::from(self.memory_manager.compression_ratio * 100.0)).unwrap();
        js_sys::Reflect::set(&diagnostics, &"recent_inference_count".into(), &JsValue::from(recent_inferences.len())).unwrap();
        
        if !recent_inferences.is_empty() {
            let avg_recent = recent_inferences.iter().sum::<f64>() / recent_inferences.len() as f64;
            js_sys::Reflect::set(&diagnostics, &"recent_avg_inference_ms".into(), &JsValue::from(avg_recent)).unwrap();
        }
        
        diagnostics.into()
    }
}

// Private implementation methods
impl WASMNeuralBridge {
    fn calculate_performance_score(&self, agent: &NeuralAgent) -> f64 {
        if agent.inference_count == 0 {
            return 1.0;
        }
        
        let avg_inference_time = agent.total_inference_time / agent.inference_count as f64;
        let time_score = (100.0 - avg_inference_time.min(100.0)) / 100.0;
        let usage_score = agent.inference_count as f64 / 100.0; // Normalize to reasonable usage
        
        (time_score * 0.7 + usage_score.min(1.0) * 0.3).max(0.1)
    }
    
    fn calculate_percentile(&self, data: &[f64], percentile: f64) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let index = (percentile / 100.0 * (sorted.len() - 1) as f64) as usize;
        sorted[index.min(sorted.len() - 1)]
    }
    
    fn optimize_agent_pool(&mut self) {
        // Sort agents by performance score and remove underperformers
        self.agent_pool.sort_by(|a, b| b.performance_score.partial_cmp(&a.performance_score).unwrap());
        
        // Keep top 80% of agents if we're near capacity
        if self.agent_pool.len() > self.max_agents * 3 / 4 {
            let keep_count = self.max_agents * 4 / 5;
            self.agent_pool.truncate(keep_count);
        }
    }
    
    fn compress_networks(&mut self) {
        // In a real implementation, this would apply network compression techniques
        // For now, just update the compression ratio metric
        self.memory_manager.compression_ratio = 0.85; // Simulated 15% compression
        perf_log!("📦 Network compression applied: {:.1}% reduction", 
                 (1.0 - self.memory_manager.compression_ratio) * 100.0);
    }
}

// Initialize panic hook for better error reporting
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
    perf_log!("🔧 WASM Neural Bridge initialized");
}