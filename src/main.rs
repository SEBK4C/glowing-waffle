use anyhow::{Context, Result};
use clap::Parser;
use std::time::Duration;

mod animation;
mod benchmark;
mod renderer;
mod terminal;
mod waffle;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable benchmarking mode
    #[arg(long)]
    benchmark: bool,

    /// Set the frames per second for the animation
    #[arg(long, default_value_t = 30)]
    fps: u8,

    /// Set the color mode (normal, rainbow, fire)
    #[arg(long, default_value = "normal")]
    color: String,

    /// Set the size of the waffle (small, medium, large)
    #[arg(long, default_value = "medium")]
    size: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize terminal
    let mut term = terminal::Terminal::new().context("Failed to initialize terminal")?;
    
    // Create waffle model based on size
    let waffle_size = match args.size.as_str() {
        "small" => waffle::Size::Small,
        "large" => waffle::Size::Large,
        _ => waffle::Size::Medium,
    };
    
    let mut waffle = waffle::Waffle::new(waffle_size);
    
    // Set up renderer with color mode
    let color_mode = match args.color.as_str() {
        "rainbow" => renderer::ColorMode::Rainbow,
        "fire" => renderer::ColorMode::Fire,
        _ => renderer::ColorMode::Normal,
    };
    
    let mut renderer = renderer::Renderer::new(color_mode);
    
    // Set up animation
    let mut animation = animation::Animation::new();
    
    // Calculate frame duration based on FPS
    let frame_duration = Duration::from_millis(1000 / args.fps as u64);
    
    // Benchmark mode
    if args.benchmark {
        return benchmark::run_benchmark(&mut term, &mut renderer, &mut waffle, &mut animation)
            .context("Benchmark failed");
    }
    
    // Main animation loop
    println!("Starting Glowing Waffle visualization. Press 'q' to quit.");
    
    term.run_event_loop(frame_duration, |term| {
        // Update waffle animation
        animation.update(&mut waffle);
        
        // Render current frame
        renderer.render(term, &waffle)?;
        
        Ok(())
    })
    .context("Animation loop failed")?;
    
    println!("Glowing Waffle visualization ended.");
    
    Ok(())
}
