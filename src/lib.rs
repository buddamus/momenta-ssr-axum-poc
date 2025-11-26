// Include WASM module only for WASM builds
#[cfg(target_arch = "wasm32")]
mod wasm;

// Components module with our refactored Counter components
pub mod components;

// Re-export components for easy access
pub use components::*;
