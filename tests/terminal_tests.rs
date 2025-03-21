#[cfg(test)]
mod tests {
    use std::time::Duration;
    
    // These tests can only be run in a manual fashion since they interact with the terminal
    // and require user input
    
    #[test]
    #[ignore]
    fn test_terminal_init() {
        // This test should be run manually since it initializes the terminal
        let terminal_result = glowing_waffle::terminal::Terminal::new();
        assert!(terminal_result.is_ok(), "Terminal initialization failed");
        
        // Terminal will be cleaned up when dropped
    }
    
    #[test]
    #[ignore]
    fn test_terminal_dimensions() {
        // This test checks that we can get terminal dimensions
        if let Ok(term) = glowing_waffle::terminal::Terminal::new() {
            assert!(term.width() > 0, "Terminal width should be greater than 0");
            assert!(term.height() > 0, "Terminal height should be greater than 0");
            
            println!("Terminal dimensions: {}x{}", term.width(), term.height());
        } else {
            panic!("Failed to initialize terminal");
        }
    }
    
    #[test]
    #[ignore]
    fn test_event_loop() {
        // This test runs a simple event loop for 2 seconds
        if let Ok(mut term) = glowing_waffle::terminal::Terminal::new() {
            let start_time = std::time::Instant::now();
            let max_duration = Duration::from_secs(2);
            
            let result = term.run_event_loop(Duration::from_millis(100), |term| {
                let elapsed = start_time.elapsed();
                
                // Print a message in the center of the terminal
                let message = format!("Test running for {:.1} seconds", elapsed.as_secs_f64());
                let x = term.width().saturating_sub(message.len() as u16) / 2;
                let y = term.height() / 2;
                
                crossterm::execute!(
                    std::io::stdout(),
                    crossterm::cursor::MoveTo(x, y),
                    crossterm::style::Print(message)
                ).unwrap();
                
                // Break the loop after max_duration
                if elapsed >= max_duration {
                    return Err(anyhow::anyhow!("Test complete"));
                }
                
                Ok(())
            });
            
            // The loop should end with our test error
            assert!(result.is_err(), "Event loop should have ended with an error");
            assert_eq!(
                result.unwrap_err().to_string(),
                "Test complete",
                "Event loop should have ended with our test message"
            );
        } else {
            panic!("Failed to initialize terminal");
        }
    }
}
