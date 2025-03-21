use std::time::{Duration, Instant};
use crate::waffle::Waffle;

/// Animation types for the waffle
#[derive(Debug, Clone, Copy)]
pub enum AnimationType {
    Rotate,
    Pulse,
    Wave,
    Bounce,
}

/// Manages animations for the waffle
pub struct Animation {
    animation_type: AnimationType,
    start_time: Instant,
    elapsed: Duration,
    rotation_speed: (f64, f64, f64),
}

impl Animation {
    pub fn new() -> Self {
        Self {
            animation_type: AnimationType::Rotate,
            start_time: Instant::now(),
            elapsed: Duration::from_secs(0),
            rotation_speed: (0.01, 0.02, 0.003),
        }
    }
    
    /// Set the animation type
    pub fn set_animation_type(&mut self, animation_type: AnimationType) {
        self.animation_type = animation_type;
    }
    
    /// Set the rotation speed
    pub fn set_rotation_speed(&mut self, x: f64, y: f64, z: f64) {
        self.rotation_speed = (x, y, z);
    }
    
    /// Get the current elapsed time in seconds
    pub fn elapsed_time(&self) -> f64 {
        self.elapsed.as_secs_f64()
    }
    
    /// Update the waffle based on the current animation
    pub fn update(&mut self, waffle: &mut Waffle) {
        // Update elapsed time
        self.elapsed = self.start_time.elapsed();
        let time = self.elapsed.as_secs_f64();
        
        match self.animation_type {
            AnimationType::Rotate => {
                // Simple rotation animation
                let (dx, dy, dz) = self.rotation_speed;
                waffle.rotate(dx, dy, dz);
            },
            AnimationType::Pulse => {
                // Pulsing animation that changes size over time
                let pulse = (time.sin() * 0.5 + 0.5) * 0.2 + 0.9;
                
                // Rotate with variable speed based on pulse
                let (dx, dy, dz) = self.rotation_speed;
                waffle.rotate(dx * pulse, dy * pulse, dz * pulse);
            },
            AnimationType::Wave => {
                // Wave-like animation
                let wave_x = (time * 0.5).sin() * 0.04;
                let wave_y = (time * 0.7).sin() * 0.03;
                let wave_z = (time * 0.3).cos() * 0.02;
                
                waffle.rotate(wave_x, wave_y, wave_z);
            },
            AnimationType::Bounce => {
                // Bouncing animation
                let bounce = ((time * 2.0).sin().abs() * 0.7 + 0.3) * 0.05;
                let (dx, dy, dz) = self.rotation_speed;
                
                waffle.rotate(dx * bounce, dy * bounce, dz * bounce);
            },
        }
    }
    
    /// Cycle to the next animation type
    pub fn next_animation(&mut self) {
        self.animation_type = match self.animation_type {
            AnimationType::Rotate => AnimationType::Pulse,
            AnimationType::Pulse => AnimationType::Wave,
            AnimationType::Wave => AnimationType::Bounce,
            AnimationType::Bounce => AnimationType::Rotate,
        };
        
        // Reset start time for the new animation
        self.start_time = Instant::now();
    }
}
