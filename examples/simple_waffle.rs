use anyhow::Result;
use glowing_waffle::{
    animation::Animation,
    renderer::{ColorMode, Renderer},
    terminal::Terminal,
    waffle::{Size, Waffle},
};
use std::time::Duration;

fn main() -> Result<()> {
    // Initialize terminal
    let mut term = Terminal::new()?;
    
    // Create a medium-sized waffle
    let mut waffle = Waffle::new(Size::Medium);
    
    // Set up renderer with rainbow color mode
    let mut renderer = Renderer::new(ColorMode::Rainbow);
    
    // Set up animation
    let mut animation = Animation::new();
    
    // Calculate frame duration based on FPS
    let frame_duration = Duration::from_millis(1000 / 30);
    
    // Main animation loop
    println!("Starting Glowing Waffle visualization. Press 'q' to quit.");
    
    term.run_event_loop(frame_duration, |term| {
        // Update waffle animation
        animation.update(&mut waffle);
        
        // Render current frame
        renderer.render(term, &waffle)?;
        
        Ok(())
    })?;
    
    println!("Glowing Waffle visualization ended.");
    
    Ok(())
} 