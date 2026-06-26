// SIMD Optimization Configuration for ruv-FANN WASM
// Enables high-performance neural inference with <100ms latency targets

#[cfg(target_arch = "wasm32")]
use std::arch::wasm32::*;

/// SIMD-accelerated neural network operations for WASM
pub struct SIMDNeuralOps;

impl SIMDNeuralOps {
    /// SIMD-accelerated dot product for neural layer computation
    #[cfg(target_feature = "simd128")]
    pub fn simd_dot_product(a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len());
        
        let mut sum = f32x4_splat(0.0);
        let chunks = a.len() / 4;
        
        // Process 4 elements at a time using SIMD
        for i in 0..chunks {
            let offset = i * 4;
            let a_chunk = v128_load(&a[offset] as *const f32 as *const v128);
            let b_chunk = v128_load(&b[offset] as *const f32 as *const v128);
            
            let product = f32x4_mul(a_chunk, b_chunk);
            sum = f32x4_add(sum, product);
        }
        
        // Sum the SIMD register
        let sum_array = [
            f32x4_extract_lane::<0>(sum),
            f32x4_extract_lane::<1>(sum),
            f32x4_extract_lane::<2>(sum),
            f32x4_extract_lane::<3>(sum),
        ];
        let mut result = sum_array.iter().sum::<f32>();
        
        // Handle remaining elements
        for i in (chunks * 4)..a.len() {
            result += a[i] * b[i];
        }
        
        result
    }
    
    /// Fallback dot product for non-SIMD environments
    #[cfg(not(target_feature = "simd128"))]
    pub fn simd_dot_product(a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len());
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }
    
    /// SIMD-accelerated matrix-vector multiplication
    #[cfg(target_feature = "simd128")]
    pub fn simd_matrix_vector_mul(matrix: &[f32], vector: &[f32], rows: usize, cols: usize) -> Vec<f32> {
        assert_eq!(matrix.len(), rows * cols);
        assert_eq!(vector.len(), cols);
        
        let mut result = vec![0.0; rows];
        
        for row in 0..rows {
            let row_start = row * cols;
            let row_slice = &matrix[row_start..row_start + cols];
            result[row] = Self::simd_dot_product(row_slice, vector);
        }
        
        result
    }
    
    /// SIMD-accelerated activation functions
    #[cfg(target_feature = "simd128")]
    pub fn simd_sigmoid(input: &mut [f32]) {
        let chunks = input.len() / 4;
        
        for i in 0..chunks {
            let offset = i * 4;
            let values = v128_load(&input[offset] as *const f32 as *const v128);
            
            // Approximate sigmoid using SIMD
            // sigmoid(x) ≈ 0.5 + 0.25 * x / (1 + |x|)
            let ones = f32x4_splat(1.0);
            let half = f32x4_splat(0.5);
            let quarter = f32x4_splat(0.25);
            
            let abs_x = f32x4_abs(values);
            let denom = f32x4_add(ones, abs_x);
            let frac = f32x4_div(values, denom);
            let result = f32x4_add(half, f32x4_mul(quarter, frac));
            
            v128_store(&mut input[offset] as *mut f32 as *mut v128, result);
        }
        
        // Handle remaining elements
        for i in (chunks * 4)..input.len() {
            input[i] = 1.0 / (1.0 + (-input[i]).exp());
        }
    }
    
    /// SIMD-accelerated ReLU activation
    #[cfg(target_feature = "simd128")]
    pub fn simd_relu(input: &mut [f32]) {
        let chunks = input.len() / 4;
        let zero = f32x4_splat(0.0);
        
        for i in 0..chunks {
            let offset = i * 4;
            let values = v128_load(&input[offset] as *const f32 as *const v128);
            let result = f32x4_pmax(values, zero);
            v128_store(&mut input[offset] as *mut f32 as *mut v128, result);
        }
        
        // Handle remaining elements
        for i in (chunks * 4)..input.len() {
            input[i] = input[i].max(0.0);
        }
    }
    
    /// SIMD-accelerated tanh activation
    #[cfg(target_feature = "simd128")]
    pub fn simd_tanh(input: &mut [f32]) {
        let chunks = input.len() / 4;
        
        for i in 0..chunks {
            let offset = i * 4;
            let values = v128_load(&input[offset] as *const f32 as *const v128);
            
            // Approximate tanh using SIMD
            // tanh(x) ≈ x / (1 + |x|) for fast approximation
            let ones = f32x4_splat(1.0);
            let abs_x = f32x4_abs(values);
            let denom = f32x4_add(ones, abs_x);
            let result = f32x4_div(values, denom);
            
            v128_store(&mut input[offset] as *mut f32 as *mut v128, result);
        }
        
        // Handle remaining elements with exact tanh
        for i in (chunks * 4)..input.len() {
            input[i] = input[i].tanh();
        }
    }
    
    /// Batch processing with SIMD optimization
    #[cfg(target_feature = "simd128")]
    pub fn simd_batch_process(
        inputs: &[&[f32]], 
        weights: &[f32], 
        biases: &[f32], 
        output_size: usize
    ) -> Vec<Vec<f32>> {
        let batch_size = inputs.len();
        let mut results = Vec::with_capacity(batch_size);
        
        for input in inputs {
            let mut output = Self::simd_matrix_vector_mul(weights, input, output_size, input.len());
            
            // Add biases using SIMD
            let chunks = output.len() / 4;
            for i in 0..chunks {
                let offset = i * 4;
                let output_chunk = v128_load(&output[offset] as *const f32 as *const v128);
                let bias_chunk = v128_load(&biases[offset] as *const f32 as *const v128);
                let result = f32x4_add(output_chunk, bias_chunk);
                v128_store(&mut output[offset] as *mut f32 as *mut v128, result);
            }
            
            // Handle remaining elements
            for i in (chunks * 4)..output.len() {
                output[i] += biases[i];
            }
            
            results.push(output);
        }
        
        results
    }
}

/// Memory-efficient neural network layer with SIMD support
pub struct SIMDNeuralLayer {
    weights: Vec<f32>,
    biases: Vec<f32>,
    input_size: usize,
    output_size: usize,
    activation_type: ActivationType,
}

#[derive(Debug, Clone, Copy)]
pub enum ActivationType {
    Sigmoid,
    ReLU,
    Tanh,
    Linear,
}

impl SIMDNeuralLayer {
    pub fn new(input_size: usize, output_size: usize, activation: ActivationType) -> Self {
        let weights = vec![0.0; input_size * output_size];
        let biases = vec![0.0; output_size];
        
        Self {
            weights,
            biases,
            input_size,
            output_size,
            activation_type: activation,
        }
    }
    
    /// Forward pass with SIMD optimization
    pub fn forward(&self, input: &[f32]) -> Vec<f32> {
        assert_eq!(input.len(), self.input_size);
        
        let mut output = SIMDNeuralOps::simd_matrix_vector_mul(
            &self.weights, 
            input, 
            self.output_size, 
            self.input_size
        );
        
        // Add biases
        for (out, bias) in output.iter_mut().zip(self.biases.iter()) {
            *out += bias;
        }
        
        // Apply activation function
        match self.activation_type {
            ActivationType::Sigmoid => SIMDNeuralOps::simd_sigmoid(&mut output),
            ActivationType::ReLU => SIMDNeuralOps::simd_relu(&mut output),
            ActivationType::Tanh => SIMDNeuralOps::simd_tanh(&mut output),
            ActivationType::Linear => {}, // No activation
        }
        
        output
    }
    
    /// Batch forward pass for multiple inputs
    pub fn forward_batch(&self, inputs: &[&[f32]]) -> Vec<Vec<f32>> {
        SIMDNeuralOps::simd_batch_process(
            inputs, 
            &self.weights, 
            &self.biases, 
            self.output_size
        )
    }
}

/// High-performance neural network optimized for WASM+SIMD
pub struct SIMDNeuralNetwork {
    layers: Vec<SIMDNeuralLayer>,
    layer_sizes: Vec<usize>,
}

impl SIMDNeuralNetwork {
    pub fn new(layer_sizes: &[usize], activations: &[ActivationType]) -> Self {
        assert!(!layer_sizes.is_empty());
        assert_eq!(layer_sizes.len() - 1, activations.len());
        
        let mut layers = Vec::new();
        
        for i in 0..layer_sizes.len() - 1 {
            let layer = SIMDNeuralLayer::new(
                layer_sizes[i], 
                layer_sizes[i + 1], 
                activations[i]
            );
            layers.push(layer);
        }
        
        Self {
            layers,
            layer_sizes: layer_sizes.to_vec(),
        }
    }
    
    /// High-performance inference with SIMD
    pub fn infer(&self, input: &[f32]) -> Vec<f32> {
        let mut current_input = input.to_vec();
        
        for layer in &self.layers {
            current_input = layer.forward(&current_input);
        }
        
        current_input
    }
    
    /// Batch inference for multiple inputs
    pub fn infer_batch(&self, inputs: &[Vec<f32>]) -> Vec<Vec<f32>> {
        let input_refs: Vec<&[f32]> = inputs.iter().map(|v| v.as_slice()).collect();
        let mut batch_outputs = input_refs;
        
        for layer in &self.layers {
            let layer_outputs = layer.forward_batch(&batch_outputs);
            // Update batch_outputs for next layer
            // This requires some memory management in real implementation
        }
        
        // Simplified return for compilation
        inputs.iter().map(|input| self.infer(input)).collect()
    }
    
    /// Get memory usage estimate
    pub fn memory_usage(&self) -> usize {
        let mut total = 0;
        
        for layer in &self.layers {
            total += layer.weights.len() * std::mem::size_of::<f32>();
            total += layer.biases.len() * std::mem::size_of::<f32>();
        }
        
        total
    }
    
    /// Get performance statistics
    pub fn get_performance_info(&self) -> PerformanceInfo {
        PerformanceInfo {
            total_layers: self.layers.len(),
            total_weights: self.layers.iter().map(|l| l.weights.len()).sum(),
            total_neurons: self.layer_sizes.iter().sum::<usize>(),
            memory_usage_bytes: self.memory_usage(),
            simd_enabled: cfg!(target_feature = "simd128"),
            bulk_memory_enabled: cfg!(target_feature = "bulk-memory"),
        }
    }
}

#[derive(Debug)]
pub struct PerformanceInfo {
    pub total_layers: usize,
    pub total_weights: usize,
    pub total_neurons: usize,
    pub memory_usage_bytes: usize,
    pub simd_enabled: bool,
    pub bulk_memory_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simd_dot_product() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![2.0, 3.0, 4.0, 5.0];
        
        let result = SIMDNeuralOps::simd_dot_product(&a, &b);
        let expected = 1.0 * 2.0 + 2.0 * 3.0 + 3.0 * 4.0 + 4.0 * 5.0; // 40.0
        
        assert!((result - expected).abs() < 1e-6);
    }
    
    #[test]
    fn test_neural_layer() {
        let layer = SIMDNeuralLayer::new(3, 2, ActivationType::ReLU);
        let input = vec![1.0, 2.0, 3.0];
        
        let output = layer.forward(&input);
        assert_eq!(output.len(), 2);
    }
    
    #[test]
    fn test_neural_network() {
        let layer_sizes = vec![3, 5, 2];
        let activations = vec![ActivationType::ReLU, ActivationType::Sigmoid];
        
        let network = SIMDNeuralNetwork::new(&layer_sizes, &activations);
        let input = vec![1.0, 2.0, 3.0];
        
        let output = network.infer(&input);
        assert_eq!(output.len(), 2);
        
        let info = network.get_performance_info();
        assert_eq!(info.total_layers, 2);
    }
    
    #[test]
    fn test_batch_inference() {
        let layer_sizes = vec![2, 3, 1];
        let activations = vec![ActivationType::Tanh, ActivationType::Sigmoid];
        
        let network = SIMDNeuralNetwork::new(&layer_sizes, &activations);
        let inputs = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
            vec![5.0, 6.0],
        ];
        
        let outputs = network.infer_batch(&inputs);
        assert_eq!(outputs.len(), 3);
        assert_eq!(outputs[0].len(), 1);
    }
}