// WASM module - WebAssembly integration for neural networks

pub mod bridge;

// Re-export main types
pub use bridge::{WASMNeuralBridge, NeuralAgent};