// Real-time Performance Monitoring for Neural Agent System
// Tracks <100ms inference latency and <50MB memory per agent targets

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// Real-time performance monitor with alerting
pub struct PerformanceMonitor {
    metrics: PerformanceMetrics,
    thresholds: PerformanceThresholds,
    alerts: Vec<PerformanceAlert>,
    sampling_window: Duration,
    max_samples: usize,
    monitoring_active: bool,
}

/// Comprehensive performance metrics collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    // Agent lifecycle metrics
    pub agent_spawn_times: VecDeque<TimedMetric>,
    pub agent_termination_times: VecDeque<TimedMetric>,
    pub active_agent_count: usize,
    
    // Neural inference metrics
    pub inference_latencies: VecDeque<InferenceMetric>,
    pub batch_inference_metrics: VecDeque<BatchInferenceMetric>,
    pub neural_accuracy_scores: VecDeque<AccuracyMetric>,
    
    // Memory metrics
    pub memory_usage_per_agent: VecDeque<MemoryMetric>,
    pub total_system_memory: VecDeque<TimedMetric>,
    pub memory_fragmentation: VecDeque<TimedMetric>,
    
    // System health metrics
    pub cpu_utilization: VecDeque<TimedMetric>,
    pub wasm_heap_usage: VecDeque<TimedMetric>,
    pub gc_frequency: VecDeque<TimedMetric>,
    
    // SIMD optimization metrics
    pub simd_effectiveness: VecDeque<SIMDMetric>,
    pub vectorization_ratio: VecDeque<TimedMetric>,
    
    // Error tracking
    pub error_rates: HashMap<String, ErrorRate>,
    pub recovery_times: VecDeque<TimedMetric>,
    
    // Performance trends
    pub hourly_aggregates: VecDeque<AggregateMetric>,
    pub daily_summaries: VecDeque<DailySummary>,
}

/// Performance thresholds for alerting
#[derive(Debug, Clone)]
pub struct PerformanceThresholds {
    pub max_agent_spawn_time_ms: f64,     // Target: <75ms
    pub max_inference_latency_ms: f64,    // Target: <100ms
    pub max_memory_per_agent_mb: f64,     // Target: <50MB
    pub min_system_health_percent: f64,   // Target: >95%
    pub max_error_rate_percent: f64,      // Target: <5%
    pub max_total_memory_gb: f64,         // Target: <2GB for 25+ agents
    pub min_simd_effectiveness: f64,      // Target: >80% SIMD utilization
    pub max_gc_frequency_per_minute: u32, // Target: <10 GC events/min
}

/// Detailed metric structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimedMetric {
    pub timestamp: u64,
    pub value: f64,
    pub agent_id: Option<String>,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceMetric {
    pub timestamp: u64,
    pub agent_id: String,
    pub latency_ms: f64,
    pub input_size: usize,
    pub output_size: usize,
    pub network_id: String,
    pub simd_used: bool,
    pub memory_allocated: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchInferenceMetric {
    pub timestamp: u64,
    pub agent_id: String,
    pub batch_size: usize,
    pub total_time_ms: f64,
    pub avg_latency_ms: f64,
    pub throughput_per_sec: f64,
    pub memory_peak: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetric {
    pub timestamp: u64,
    pub agent_id: String,
    pub heap_usage_bytes: usize,
    pub stack_usage_bytes: usize,
    pub network_memory_bytes: usize,
    pub total_allocated_bytes: usize,
    pub fragmentation_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccuracyMetric {
    pub timestamp: u64,
    pub agent_id: String,
    pub accuracy_score: f64,
    pub confidence_score: f64,
    pub test_set_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIMDMetric {
    pub timestamp: u64,
    pub operation_type: String,
    pub simd_time_ms: f64,
    pub scalar_time_ms: f64,
    pub speedup_factor: f64,
    pub vectorization_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRate {
    pub error_type: String,
    pub count: u32,
    pub last_occurrence: u64,
    pub rate_per_hour: f64,
    pub severity: AlertSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateMetric {
    pub hour_timestamp: u64,
    pub avg_inference_latency: f64,
    pub max_inference_latency: f64,
    pub total_inferences: u32,
    pub avg_memory_usage: f64,
    pub peak_memory_usage: f64,
    pub error_count: u32,
    pub agent_spawn_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySummary {
    pub date: String,
    pub total_inferences: u32,
    pub avg_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub peak_agents: usize,
    pub peak_memory_gb: f64,
    pub uptime_percent: f64,
    pub error_rate_percent: f64,
    pub performance_score: f64,
}

/// Performance alerts with severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub timestamp: u64,
    pub severity: AlertSeverity,
    pub alert_type: AlertType,
    pub message: String,
    pub current_value: f64,
    pub threshold_value: f64,
    pub agent_id: Option<String>,
    pub suggested_action: String,
    pub resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    InferenceLatency,
    MemoryUsage,
    AgentSpawnTime,
    SystemHealth,
    ErrorRate,
    SIMDEffectiveness,
    MemoryFragmentation,
    GCPressure,
}

impl PerformanceMonitor {
    /// Create a new performance monitor with default thresholds
    pub fn new() -> Self {
        Self {
            metrics: PerformanceMetrics::new(),
            thresholds: PerformanceThresholds::default(),
            alerts: Vec::new(),
            sampling_window: Duration::from_secs(300), // 5 minutes
            max_samples: 1000,
            monitoring_active: true,
        }
    }
    
    /// Start monitoring with custom thresholds
    pub fn with_thresholds(thresholds: PerformanceThresholds) -> Self {
        Self {
            thresholds,
            ..Self::new()
        }
    }
    
    /// Record agent spawn performance
    pub fn record_agent_spawn(&mut self, agent_id: &str, spawn_time_ms: f64) {
        let timestamp = current_timestamp();
        
        self.metrics.agent_spawn_times.push_back(TimedMetric {
            timestamp,
            value: spawn_time_ms,
            agent_id: Some(agent_id.to_string()),
            context: Some("agent_spawn".to_string()),
        });
        
        // Check threshold
        if spawn_time_ms > self.thresholds.max_agent_spawn_time_ms {
            self.create_alert(
                AlertSeverity::Warning,
                AlertType::AgentSpawnTime,
                format!("Agent spawn time {:.2}ms exceeds threshold {:.2}ms", 
                       spawn_time_ms, self.thresholds.max_agent_spawn_time_ms),
                spawn_time_ms,
                self.thresholds.max_agent_spawn_time_ms,
                Some(agent_id.to_string()),
                "Consider optimizing agent initialization or reducing system load".to_string(),
            );
        }
        
        self.cleanup_old_samples(&mut self.metrics.agent_spawn_times);
    }
    
    /// Record neural inference performance
    pub fn record_inference(&mut self, 
                           agent_id: &str, 
                           network_id: &str,
                           latency_ms: f64, 
                           input_size: usize, 
                           output_size: usize,
                           simd_used: bool,
                           memory_allocated: usize) {
        let timestamp = current_timestamp();
        
        let metric = InferenceMetric {
            timestamp,
            agent_id: agent_id.to_string(),
            latency_ms,
            input_size,
            output_size,
            network_id: network_id.to_string(),
            simd_used,
            memory_allocated,
        };
        
        self.metrics.inference_latencies.push_back(metric);
        
        // Check latency threshold
        if latency_ms > self.thresholds.max_inference_latency_ms {
            let severity = if latency_ms > self.thresholds.max_inference_latency_ms * 2.0 {
                AlertSeverity::Critical
            } else {
                AlertSeverity::Warning
            };
            
            self.create_alert(
                severity,
                AlertType::InferenceLatency,
                format!("Inference latency {:.2}ms exceeds threshold {:.2}ms", 
                       latency_ms, self.thresholds.max_inference_latency_ms),
                latency_ms,
                self.thresholds.max_inference_latency_ms,
                Some(agent_id.to_string()),
                if simd_used {
                    "Consider network optimization or reducing model complexity".to_string()
                } else {
                    "Enable SIMD optimization for better performance".to_string()
                },
            );
        }
        
        self.cleanup_old_samples(&mut self.metrics.inference_latencies);
    }
    
    /// Record batch inference performance
    pub fn record_batch_inference(&mut self,
                                 agent_id: &str,
                                 batch_size: usize,
                                 total_time_ms: f64,
                                 memory_peak: usize) {
        let timestamp = current_timestamp();
        let avg_latency_ms = total_time_ms / batch_size as f64;
        let throughput_per_sec = 1000.0 * batch_size as f64 / total_time_ms;
        
        let metric = BatchInferenceMetric {
            timestamp,
            agent_id: agent_id.to_string(),
            batch_size,
            total_time_ms,
            avg_latency_ms,
            throughput_per_sec,
            memory_peak,
        };
        
        self.metrics.batch_inference_metrics.push_back(metric);
        
        // Check if batch processing is efficient
        if avg_latency_ms > self.thresholds.max_inference_latency_ms {
            self.create_alert(
                AlertSeverity::Info,
                AlertType::InferenceLatency,
                format!("Batch inference average latency {:.2}ms exceeds single inference threshold", avg_latency_ms),
                avg_latency_ms,
                self.thresholds.max_inference_latency_ms,
                Some(agent_id.to_string()),
                "Consider optimizing batch processing or reducing batch size".to_string(),
            );
        }
    }
    
    /// Record memory usage for an agent
    pub fn record_memory_usage(&mut self,
                              agent_id: &str,
                              heap_usage: usize,
                              stack_usage: usize,
                              network_memory: usize) {
        let timestamp = current_timestamp();
        let total_allocated = heap_usage + stack_usage + network_memory;
        let total_mb = total_allocated as f64 / 1024.0 / 1024.0;
        
        let metric = MemoryMetric {
            timestamp,
            agent_id: agent_id.to_string(),
            heap_usage_bytes: heap_usage,
            stack_usage_bytes: stack_usage,
            network_memory_bytes: network_memory,
            total_allocated_bytes: total_allocated,
            fragmentation_ratio: calculate_fragmentation_ratio(heap_usage, total_allocated),
        };
        
        self.metrics.memory_usage_per_agent.push_back(metric);
        
        // Check memory threshold
        if total_mb > self.thresholds.max_memory_per_agent_mb {
            let severity = if total_mb > self.thresholds.max_memory_per_agent_mb * 2.0 {
                AlertSeverity::Critical
            } else {
                AlertSeverity::Warning
            };
            
            self.create_alert(
                severity,
                AlertType::MemoryUsage,
                format!("Agent memory usage {:.2}MB exceeds threshold {:.2}MB", 
                       total_mb, self.thresholds.max_memory_per_agent_mb),
                total_mb,
                self.thresholds.max_memory_per_agent_mb,
                Some(agent_id.to_string()),
                "Consider memory cleanup or reducing network complexity".to_string(),
            );
        }
        
        self.cleanup_old_samples(&mut self.metrics.memory_usage_per_agent);
    }
    
    /// Record SIMD optimization effectiveness
    pub fn record_simd_performance(&mut self,
                                  operation_type: &str,
                                  simd_time_ms: f64,
                                  scalar_time_ms: f64,
                                  vectorization_percent: f64) {
        let timestamp = current_timestamp();
        let speedup_factor = scalar_time_ms / simd_time_ms;
        
        let metric = SIMDMetric {
            timestamp,
            operation_type: operation_type.to_string(),
            simd_time_ms,
            scalar_time_ms,
            speedup_factor,
            vectorization_percent,
        };
        
        self.metrics.simd_effectiveness.push_back(metric);
        
        // Check SIMD effectiveness
        if vectorization_percent < self.thresholds.min_simd_effectiveness {
            self.create_alert(
                AlertSeverity::Info,
                AlertType::SIMDEffectiveness,
                format!("SIMD vectorization {:.1}% below threshold {:.1}%", 
                       vectorization_percent, self.thresholds.min_simd_effectiveness),
                vectorization_percent,
                self.thresholds.min_simd_effectiveness,
                None,
                "Check browser SIMD support or optimize vector operations".to_string(),
            );
        }
    }
    
    /// Get current performance snapshot
    pub fn get_current_metrics(&self) -> PerformanceSnapshot {
        let now = current_timestamp();
        let window_start = now - self.sampling_window.as_secs() * 1000;
        
        // Calculate recent averages
        let recent_spawn_times: Vec<f64> = self.metrics.agent_spawn_times
            .iter()
            .filter(|m| m.timestamp >= window_start)
            .map(|m| m.value)
            .collect();
        
        let recent_inference_latencies: Vec<f64> = self.metrics.inference_latencies
            .iter()
            .filter(|m| m.timestamp >= window_start)
            .map(|m| m.latency_ms)
            .collect();
        
        let recent_memory_usage: Vec<f64> = self.metrics.memory_usage_per_agent
            .iter()
            .filter(|m| m.timestamp >= window_start)
            .map(|m| m.total_allocated_bytes as f64 / 1024.0 / 1024.0)
            .collect();
        
        PerformanceSnapshot {
            timestamp: now,
            active_agents: self.metrics.active_agent_count,
            avg_spawn_time_ms: calculate_average(&recent_spawn_times),
            avg_inference_latency_ms: calculate_average(&recent_inference_latencies),
            p95_inference_latency_ms: calculate_percentile(&recent_inference_latencies, 95.0),
            p99_inference_latency_ms: calculate_percentile(&recent_inference_latencies, 99.0),
            avg_memory_per_agent_mb: calculate_average(&recent_memory_usage),
            peak_memory_usage_mb: recent_memory_usage.iter().fold(0.0, |a, &b| a.max(b)),
            total_inferences: recent_inference_latencies.len(),
            active_alerts: self.alerts.iter().filter(|a| !a.resolved).count(),
            performance_score: self.calculate_performance_score(),
            thresholds_met: self.check_all_thresholds(),
        }
    }
    
    /// Get unresolved alerts
    pub fn get_active_alerts(&self) -> Vec<&PerformanceAlert> {
        self.alerts.iter().filter(|a| !a.resolved).collect()
    }
    
    /// Resolve an alert by timestamp
    pub fn resolve_alert(&mut self, timestamp: u64) -> bool {
        if let Some(alert) = self.alerts.iter_mut().find(|a| a.timestamp == timestamp) {
            alert.resolved = true;
            true
        } else {
            false
        }
    }
    
    /// Generate performance report
    pub fn generate_report(&self, format: ReportFormat) -> String {
        match format {
            ReportFormat::Json => self.generate_json_report(),
            ReportFormat::Text => self.generate_text_report(),
            ReportFormat::Html => self.generate_html_report(),
        }
    }
    
    /// Export metrics for external analysis
    pub fn export_metrics(&self) -> ExportedMetrics {
        ExportedMetrics {
            timestamp: current_timestamp(),
            agent_spawn_times: self.metrics.agent_spawn_times.clone(),
            inference_latencies: self.metrics.inference_latencies.clone(),
            memory_metrics: self.metrics.memory_usage_per_agent.clone(),
            simd_metrics: self.metrics.simd_effectiveness.clone(),
            alerts: self.alerts.clone(),
            thresholds: self.thresholds.clone(),
        }
    }
}

// Implementation of private methods
impl PerformanceMonitor {
    fn create_alert(&mut self,
                   severity: AlertSeverity,
                   alert_type: AlertType,
                   message: String,
                   current_value: f64,
                   threshold_value: f64,
                   agent_id: Option<String>,
                   suggested_action: String) {
        let alert = PerformanceAlert {
            timestamp: current_timestamp(),
            severity,
            alert_type,
            message,
            current_value,
            threshold_value,
            agent_id,
            suggested_action,
            resolved: false,
        };
        
        self.alerts.push(alert);
        
        // Keep only recent alerts (last 24 hours)
        let cutoff = current_timestamp() - 24 * 60 * 60 * 1000;
        self.alerts.retain(|a| a.timestamp >= cutoff);
    }
    
    fn cleanup_old_samples<T>(&self, samples: &mut VecDeque<T>)
    where
        T: HasTimestamp,
    {
        let cutoff = current_timestamp() - self.sampling_window.as_secs() * 1000;
        while let Some(front) = samples.front() {
            if front.timestamp() >= cutoff {
                break;
            }
            samples.pop_front();
        }
        
        // Also limit by count
        while samples.len() > self.max_samples {
            samples.pop_front();
        }
    }
    
    fn calculate_performance_score(&self) -> f64 {
        let mut score = 100.0;
        let recent_alerts = self.alerts.iter()
            .filter(|a| !a.resolved && a.timestamp >= current_timestamp() - 3600000) // Last hour
            .count();
        
        // Deduct points for active alerts
        score -= recent_alerts as f64 * 5.0;
        
        // Deduct points for threshold violations
        let snapshot = self.get_current_metrics();
        if snapshot.avg_inference_latency_ms > self.thresholds.max_inference_latency_ms {
            score -= 15.0;
        }
        if snapshot.avg_memory_per_agent_mb > self.thresholds.max_memory_per_agent_mb {
            score -= 10.0;
        }
        if snapshot.avg_spawn_time_ms > self.thresholds.max_agent_spawn_time_ms {
            score -= 5.0;
        }
        
        score.max(0.0).min(100.0)
    }
    
    fn check_all_thresholds(&self) -> ThresholdStatus {
        let snapshot = self.get_current_metrics();
        
        ThresholdStatus {
            spawn_time_ok: snapshot.avg_spawn_time_ms <= self.thresholds.max_agent_spawn_time_ms,
            inference_latency_ok: snapshot.p95_inference_latency_ms <= self.thresholds.max_inference_latency_ms,
            memory_usage_ok: snapshot.avg_memory_per_agent_mb <= self.thresholds.max_memory_per_agent_mb,
            error_rate_ok: true, // TODO: Calculate from error metrics
            overall_health_ok: snapshot.performance_score >= self.thresholds.min_system_health_percent,
        }
    }
    
    fn generate_json_report(&self) -> String {
        serde_json::to_string_pretty(&self.export_metrics()).unwrap_or_else(|_| "{}".to_string())
    }
    
    fn generate_text_report(&self) -> String {
        let snapshot = self.get_current_metrics();
        let active_alerts = self.get_active_alerts();
        
        format!(
            "Performance Monitor Report\n\
             Generated: {}\n\
             \n\
             Current Metrics:\n\
             - Active Agents: {}\n\
             - Avg Spawn Time: {:.2}ms (threshold: {:.2}ms)\n\
             - Avg Inference Latency: {:.2}ms (threshold: {:.2}ms)\n\
             - P95 Inference Latency: {:.2}ms\n\
             - Avg Memory per Agent: {:.2}MB (threshold: {:.2}MB)\n\
             - Performance Score: {:.1}%\n\
             \n\
             Active Alerts: {}\n\
             {}\n",
            chrono::DateTime::from_timestamp(snapshot.timestamp as i64 / 1000, 0)
                .unwrap_or_default(),
            snapshot.active_agents,
            snapshot.avg_spawn_time_ms,
            self.thresholds.max_agent_spawn_time_ms,
            snapshot.avg_inference_latency_ms,
            self.thresholds.max_inference_latency_ms,
            snapshot.p95_inference_latency_ms,
            snapshot.avg_memory_per_agent_mb,
            self.thresholds.max_memory_per_agent_mb,
            snapshot.performance_score,
            active_alerts.len(),
            active_alerts.iter()
                .map(|a| format!("  - {:?}: {}", a.severity, a.message))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
    
    fn generate_html_report(&self) -> String {
        // Simplified HTML report - in practice, this would use a template engine
        let snapshot = self.get_current_metrics();
        format!(
            "<html><body>\
             <h1>Performance Monitor Report</h1>\
             <p>Performance Score: {:.1}%</p>\
             <p>Active Agents: {}</p>\
             <p>Avg Inference Latency: {:.2}ms</p>\
             </body></html>",
            snapshot.performance_score,
            snapshot.active_agents,
            snapshot.avg_inference_latency_ms
        )
    }
}

// Supporting structures and implementations
#[derive(Debug, Clone)]
pub struct PerformanceSnapshot {
    pub timestamp: u64,
    pub active_agents: usize,
    pub avg_spawn_time_ms: f64,
    pub avg_inference_latency_ms: f64,
    pub p95_inference_latency_ms: f64,
    pub p99_inference_latency_ms: f64,
    pub avg_memory_per_agent_mb: f64,
    pub peak_memory_usage_mb: f64,
    pub total_inferences: usize,
    pub active_alerts: usize,
    pub performance_score: f64,
    pub thresholds_met: ThresholdStatus,
}

#[derive(Debug, Clone)]
pub struct ThresholdStatus {
    pub spawn_time_ok: bool,
    pub inference_latency_ok: bool,
    pub memory_usage_ok: bool,
    pub error_rate_ok: bool,
    pub overall_health_ok: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedMetrics {
    pub timestamp: u64,
    pub agent_spawn_times: VecDeque<TimedMetric>,
    pub inference_latencies: VecDeque<InferenceMetric>,
    pub memory_metrics: VecDeque<MemoryMetric>,
    pub simd_metrics: VecDeque<SIMDMetric>,
    pub alerts: Vec<PerformanceAlert>,
    pub thresholds: PerformanceThresholds,
}

pub enum ReportFormat {
    Json,
    Text,
    Html,
}

// Trait for timestamp extraction
trait HasTimestamp {
    fn timestamp(&self) -> u64;
}

impl HasTimestamp for TimedMetric {
    fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

impl HasTimestamp for InferenceMetric {
    fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

impl HasTimestamp for MemoryMetric {
    fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

impl HasTimestamp for SIMDMetric {
    fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

// Default implementations
impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_agent_spawn_time_ms: 75.0,
            max_inference_latency_ms: 100.0,
            max_memory_per_agent_mb: 50.0,
            min_system_health_percent: 95.0,
            max_error_rate_percent: 5.0,
            max_total_memory_gb: 2.0,
            min_simd_effectiveness: 80.0,
            max_gc_frequency_per_minute: 10,
        }
    }
}

impl PerformanceMetrics {
    fn new() -> Self {
        Self {
            agent_spawn_times: VecDeque::with_capacity(1000),
            agent_termination_times: VecDeque::with_capacity(1000),
            active_agent_count: 0,
            inference_latencies: VecDeque::with_capacity(10000),
            batch_inference_metrics: VecDeque::with_capacity(1000),
            neural_accuracy_scores: VecDeque::with_capacity(1000),
            memory_usage_per_agent: VecDeque::with_capacity(1000),
            total_system_memory: VecDeque::with_capacity(1000),
            memory_fragmentation: VecDeque::with_capacity(1000),
            cpu_utilization: VecDeque::with_capacity(1000),
            wasm_heap_usage: VecDeque::with_capacity(1000),
            gc_frequency: VecDeque::with_capacity(1000),
            simd_effectiveness: VecDeque::with_capacity(1000),
            vectorization_ratio: VecDeque::with_capacity(1000),
            error_rates: HashMap::new(),
            recovery_times: VecDeque::with_capacity(1000),
            hourly_aggregates: VecDeque::with_capacity(168), // 1 week of hourly data
            daily_summaries: VecDeque::with_capacity(30),    // 30 days
        }
    }
}

// Utility functions
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn calculate_average(values: &[f64]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

fn calculate_percentile(values: &[f64], percentile: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let index = (percentile / 100.0 * (sorted.len() - 1) as f64) as usize;
    sorted[index.min(sorted.len() - 1)]
}

fn calculate_fragmentation_ratio(heap_usage: usize, total_allocated: usize) -> f64 {
    if total_allocated == 0 {
        0.0
    } else {
        1.0 - (heap_usage as f64 / total_allocated as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_monitor_creation() {
        let monitor = PerformanceMonitor::new();
        assert!(monitor.monitoring_active);
        assert_eq!(monitor.alerts.len(), 0);
    }
    
    #[test]
    fn test_metric_recording() {
        let mut monitor = PerformanceMonitor::new();
        
        // Record a normal spawn time
        monitor.record_agent_spawn("test_agent", 50.0);
        assert_eq!(monitor.metrics.agent_spawn_times.len(), 1);
        assert_eq!(monitor.alerts.len(), 0);
        
        // Record a slow spawn time
        monitor.record_agent_spawn("slow_agent", 150.0);
        assert_eq!(monitor.alerts.len(), 1);
        assert!(matches!(monitor.alerts[0].alert_type, AlertType::AgentSpawnTime));
    }
    
    #[test]
    fn test_performance_calculation() {
        let mut monitor = PerformanceMonitor::new();
        monitor.record_inference("test", "net1", 50.0, 10, 5, true, 1024);
        
        let snapshot = monitor.get_current_metrics();
        assert_eq!(snapshot.avg_inference_latency_ms, 50.0);
        assert!(snapshot.thresholds_met.inference_latency_ok);
    }
    
    #[test]
    fn test_percentile_calculation() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        assert_eq!(calculate_percentile(&values, 50.0), 5.0);
        assert_eq!(calculate_percentile(&values, 95.0), 9.0);
    }
}