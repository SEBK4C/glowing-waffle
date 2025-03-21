use anyhow::Result;
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, SetForegroundColor},
};

use crate::{
    animation::Animation,
    renderer::Renderer,
    terminal::Terminal,
    waffle::{Size, Waffle},
};

/// Number of frames to render in benchmark
const BENCHMARK_FRAMES: usize = 100;

/// Run a performance benchmark
pub fn run_benchmark(
    term: &mut Terminal,
    renderer: &mut Renderer,
    waffle: &mut Waffle,
    animation: &mut Animation,
) -> Result<()> {
    println!("Running benchmark...");
    
    // First, warm up the system
    for _ in 0..10 {
        animation.update(waffle);
        renderer.render(term, waffle)?;
    }
    
    // Benchmark different waffle sizes
    let sizes = [Size::Small, Size::Medium, Size::Large];
    let size_names = ["Small", "Medium", "Large"];
    
    for (i, &size) in sizes.iter().enumerate() {
        // Create a new waffle with the current size
        let mut test_waffle = Waffle::new(size);
        
        // Set initial rotation
        test_waffle.set_rotation(0.5, 0.2, 0.1);
        
        // Time the rendering of frames
        let start = Instant::now();
        let mut frame_times = Vec::with_capacity(BENCHMARK_FRAMES);
        
        for _ in 0..BENCHMARK_FRAMES {
            let frame_start = Instant::now();
            
            // Update animation
            animation.update(&mut test_waffle);
            
            // Render frame
            renderer.render(term, &test_waffle)?;
            
            // Record frame time
            frame_times.push(frame_start.elapsed());
            
            // Small delay to avoid terminal issues
            std::thread::sleep(Duration::from_millis(5));
        }
        
        // Calculate statistics
        let total_time = start.elapsed();
        let fps = BENCHMARK_FRAMES as f64 / total_time.as_secs_f64();
        
        let avg_frame_time: Duration = frame_times.iter().sum::<Duration>() / frame_times.len() as u32;
        
        // Find min and max frame times - create owned values to fix temporary value issue
        let zero_duration = Duration::from_secs(0);
        let min_frame_time = frame_times.iter().min().unwrap_or(&zero_duration).clone();
        let max_frame_time = frame_times.iter().max().unwrap_or(&zero_duration).clone();
        
        // Clear terminal for results
        term.clear()?;
        
        // Print benchmark results
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        
        execute!(
            stdout,
            MoveTo(2, 2),
            SetForegroundColor(Color::Green),
            Print(format!("Benchmark Results for {} Waffle:", size_names[i]))
        )?;
        
        execute!(
            stdout,
            MoveTo(2, 4),
            SetForegroundColor(Color::White),
            Print(format!("Total time: {:.2} seconds", total_time.as_secs_f64()))
        )?;
        
        execute!(
            stdout,
            MoveTo(2, 5),
            SetForegroundColor(Color::White),
            Print(format!("Average FPS: {:.2}", fps))
        )?;
        
        execute!(
            stdout,
            MoveTo(2, 6),
            SetForegroundColor(Color::White),
            Print(format!("Average frame time: {:.2} ms", avg_frame_time.as_micros() as f64 / 1000.0))
        )?;
        
        execute!(
            stdout,
            MoveTo(2, 7),
            SetForegroundColor(Color::White),
            Print(format!("Min frame time: {:.2} ms", min_frame_time.as_micros() as f64 / 1000.0))
        )?;
        
        execute!(
            stdout,
            MoveTo(2, 8),
            SetForegroundColor(Color::White),
            Print(format!("Max frame time: {:.2} ms", max_frame_time.as_micros() as f64 / 1000.0))
        )?;
        
        execute!(
            stdout,
            MoveTo(2, 10),
            SetForegroundColor(Color::Yellow),
            Print("Press any key to continue to next benchmark...")
        )?;
        
        // Wait for user input before continuing
        stdout.flush()?;
        crossterm::event::read()?;
    }
    
    Ok(())
}

/// Benchmark a single frame render
pub fn benchmark_frame(
    term: &mut Terminal,
    renderer: &mut Renderer,
    waffle: &mut Waffle,
) -> Duration {
    let start = Instant::now();
    let _ = renderer.render(term, waffle);
    start.elapsed()
}
