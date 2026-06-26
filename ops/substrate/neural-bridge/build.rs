// Build script for ruv-FANN Neural Bridge
// Handles WASM optimization, SIMD detection, and build-time configuration

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Set build-time environment variables
    set_build_info();
    
    // Configure target-specific optimizations
    configure_target_optimizations();
    
    // Generate SIMD configuration
    generate_simd_config();
    
    // Setup WASM-specific configuration
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() == "wasm32" {
        configure_wasm_build();
    }
    
    // Copy or generate required files
    setup_build_files();
    
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=simd-config.rs");
    println!("cargo:rerun-if-changed=wasm-build.toml");
    println!("cargo:rerun-if-env-changed=RUSTFLAGS");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");
}

fn set_build_info() {
    // Build timestamp
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);
    
    // Git hash (if available)
    let git_hash = get_git_hash().unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    
    // Build configuration
    let profile = env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_PROFILE={}", profile);
    
    // Target information
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_TARGET={}", target);
    
    // Feature detection
    let features = get_enabled_features();
    println!("cargo:rustc-env=BUILD_FEATURES={}", features.join(","));
}

fn get_git_hash() -> Option<String> {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()?;
    
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

fn get_enabled_features() -> Vec<String> {
    let mut features = Vec::new();
    
    // Check for feature flags
    if env::var("CARGO_FEATURE_SIMD").is_ok() {
        features.push("simd".to_string());
    }
    if env::var("CARGO_FEATURE_PERFORMANCE_MONITORING").is_ok() {
        features.push("performance-monitoring".to_string());
    }
    if env::var("CARGO_FEATURE_NEURAL_OPTIMIZATION").is_ok() {
        features.push("neural-optimization".to_string());
    }
    if env::var("CARGO_FEATURE_MEMORY_POOLING").is_ok() {
        features.push("memory-pooling".to_string());
    }
    if env::var("CARGO_FEATURE_BATCH_INFERENCE").is_ok() {
        features.push("batch-inference".to_string());
    }
    if env::var("CARGO_FEATURE_DEBUG_LOGGING").is_ok() {
        features.push("debug-logging".to_string());
    }
    if env::var("CARGO_FEATURE_BENCHMARK_MODE").is_ok() {
        features.push("benchmark-mode".to_string());
    }
    
    features
}

fn configure_target_optimizations() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    match target_arch.as_str() {
        "wasm32" => {
            println!("cargo:rustc-cfg=target_wasm");
            
            // Enable WASM-specific optimizations
            if env::var("CARGO_FEATURE_SIMD").is_ok() {
                println!("cargo:rustc-cfg=wasm_simd");
            }
            
            // Set WASM memory configuration
            println!("cargo:rustc-link-arg=--initial-memory=16777216");  // 16MB initial
            println!("cargo:rustc-link-arg=--max-memory=134217728");     // 128MB max
            
            // Enable bulk memory operations
            println!("cargo:rustc-link-arg=--enable-bulk-memory");
            
            // Enable reference types
            println!("cargo:rustc-link-arg=--enable-reference-types");
        },
        "x86_64" => {
            println!("cargo:rustc-cfg=target_x86_64");
            
            // Enable x86_64 specific optimizations
            if target_os == "linux" || target_os == "macos" || target_os == "windows" {
                println!("cargo:rustc-cfg=native_simd");
            }
        },
        "aarch64" => {
            println!("cargo:rustc-cfg=target_aarch64");
            println!("cargo:rustc-cfg=native_simd"); // ARM NEON support
        },
        _ => {
            println!("cargo:rustc-cfg=generic_target");
        }
    }
}

fn generate_simd_config() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("simd_features.rs");
    
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let simd_enabled = env::var("CARGO_FEATURE_SIMD").is_ok();
    
    let simd_config = match (target_arch.as_str(), simd_enabled) {
        ("wasm32", true) => {
            r#"
// WASM SIMD configuration
pub const SIMD_AVAILABLE: bool = true;
pub const SIMD_WIDTH: usize = 16; // 128-bit SIMD
pub const VECTOR_F32_COUNT: usize = 4;
pub const VECTOR_F64_COUNT: usize = 2;

#[cfg(target_feature = "simd128")]
pub mod wasm_simd {
    pub use std::arch::wasm32::*;
    pub const SIMD_ENABLED: bool = true;
}

#[cfg(not(target_feature = "simd128"))]
pub mod wasm_simd {
    pub const SIMD_ENABLED: bool = false;
}
"#
        },
        ("x86_64", true) => {
            r#"
// x86_64 SIMD configuration
pub const SIMD_AVAILABLE: bool = true;
pub const SIMD_WIDTH: usize = 32; // 256-bit AVX
pub const VECTOR_F32_COUNT: usize = 8;
pub const VECTOR_F64_COUNT: usize = 4;

#[cfg(target_feature = "avx2")]
pub mod native_simd {
    pub use std::arch::x86_64::*;
    pub const AVX2_ENABLED: bool = true;
}

#[cfg(target_feature = "sse4.1")]
pub mod sse_simd {
    pub use std::arch::x86_64::*;
    pub const SSE41_ENABLED: bool = true;
}
"#
        },
        ("aarch64", true) => {
            r#"
// ARM NEON configuration
pub const SIMD_AVAILABLE: bool = true;
pub const SIMD_WIDTH: usize = 16; // 128-bit NEON
pub const VECTOR_F32_COUNT: usize = 4;
pub const VECTOR_F64_COUNT: usize = 2;

#[cfg(target_feature = "neon")]
pub mod neon_simd {
    pub use std::arch::aarch64::*;
    pub const NEON_ENABLED: bool = true;
}
"#
        },
        _ => {
            r#"
// Generic/fallback configuration
pub const SIMD_AVAILABLE: bool = false;
pub const SIMD_WIDTH: usize = 8; // Fallback scalar processing
pub const VECTOR_F32_COUNT: usize = 1;
pub const VECTOR_F64_COUNT: usize = 1;
"#
        }
    };
    
    fs::write(&dest_path, simd_config).unwrap();
    println!("cargo:rustc-cfg=simd_config_generated");
}

fn configure_wasm_build() {
    // Set WASM-specific link arguments for optimization
    println!("cargo:rustc-link-arg=--import-memory");
    println!("cargo:rustc-link-arg=--export-dynamic");
    
    // Optimize for size
    if env::var("PROFILE").unwrap_or_default() == "release" {
        println!("cargo:rustc-link-arg=--lto-O3");
        println!("cargo:rustc-link-arg=--strip-all");
    }
    
    // Enable shared memory (where supported)
    if env::var("WASM_SHARED_MEMORY").is_ok() {
        println!("cargo:rustc-link-arg=--shared-memory");
        println!("cargo:rustc-link-arg=--max-memory=2147483648"); // 2GB for shared memory
    }
    
    // Set stack size
    println!("cargo:rustc-link-arg=--stack-first");
    println!("cargo:rustc-link-arg=-z");
    println!("cargo:rustc-link-arg=stack-size=1048576"); // 1MB stack
}

fn setup_build_files() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Copy SIMD configuration if it exists
    if Path::new("simd-config.rs").exists() {
        let dest = Path::new(&out_dir).join("simd-config.rs");
        fs::copy("simd-config.rs", &dest).unwrap();
    }
    
    // Generate build configuration file
    let build_config = format!(
        r#"
// Auto-generated build configuration
pub const BUILD_INFO: &str = r#"
ruv-FANN Neural Bridge Build Information
========================================
Version: {}
Target: {}
Profile: {}
Features: {}
Timestamp: {}
Git Hash: {}
"#;

pub fn print_build_info() {{
    println!("{{}}", BUILD_INFO);
}}
"#,
        env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string()),
        env::var("TARGET").unwrap_or_else(|_| "unknown".to_string()),
        env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string()),
        get_enabled_features().join(", "),
        env::var("BUILD_TIMESTAMP").unwrap_or_else(|_| "unknown".to_string()),
        env::var("GIT_HASH").unwrap_or_else(|_| "unknown".to_string()),
    );
    
    let build_config_path = Path::new(&out_dir).join("build_config.rs");
    fs::write(&build_config_path, build_config).unwrap();
}

// Helper function to detect CPU features at build time
fn detect_cpu_features() -> Vec<String> {
    let mut features = Vec::new();
    
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse") {
            features.push("sse".to_string());
        }
        if is_x86_feature_detected!("sse2") {
            features.push("sse2".to_string());
        }
        if is_x86_feature_detected!("sse3") {
            features.push("sse3".to_string());
        }
        if is_x86_feature_detected!("sse4.1") {
            features.push("sse4.1".to_string());
        }
        if is_x86_feature_detected!("sse4.2") {
            features.push("sse4.2".to_string());
        }
        if is_x86_feature_detected!("avx") {
            features.push("avx".to_string());
        }
        if is_x86_feature_detected!("avx2") {
            features.push("avx2".to_string());
        }
        if is_x86_feature_detected!("fma") {
            features.push("fma".to_string());
        }
    }
    
    #[cfg(target_arch = "aarch64")]
    {
        if std::arch::is_aarch64_feature_detected!("neon") {
            features.push("neon".to_string());
        }
    }
    
    features
}

// Output optimization recommendations based on target
fn output_optimization_recommendations() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let profile = env::var("PROFILE").unwrap_or_default();
    
    if profile == "release" {
        match target_arch.as_str() {
            "wasm32" => {
                println!("cargo:warning=WASM Release Build - Consider using wasm-opt for additional optimizations");
                println!("cargo:warning=Recommended: wasm-opt -Oz --enable-simd --strip-debug target.wasm");
            },
            "x86_64" => {
                let cpu_features = detect_cpu_features();
                if !cpu_features.is_empty() {
                    println!("cargo:warning=Detected CPU features: {}", cpu_features.join(", "));
                    println!("cargo:warning=Consider setting RUSTFLAGS=\"-C target-cpu=native\" for optimal performance");
                }
            },
            _ => {}
        }
    }
}