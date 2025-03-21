//! Glowing Waffle - A terminal-based 3D ASCII art visualizer
//!
//! This library provides functionality to render a 3D waffle in the terminal
//! using ASCII characters with animation and color effects.

pub mod animation;
pub mod benchmark;
pub mod renderer;
pub mod terminal;
pub mod waffle;

// Re-export common types for easier access
pub use animation::Animation;
pub use renderer::{ColorMode, Renderer};
pub use terminal::Terminal;
pub use waffle::{Size, Waffle}; 