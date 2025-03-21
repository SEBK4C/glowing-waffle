use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Hide, Show},
};
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

/// Terminal handler for managing the terminal state and user input
pub struct Terminal {
    width: u16,
    height: u16,
}

impl Terminal {
    /// Create a new terminal instance and initialize the terminal
    pub fn new() -> Result<Self> {
        // Enter alternate screen
        execute!(io::stdout(), EnterAlternateScreen, Hide)?;
        
        // Enable raw mode
        terminal::enable_raw_mode().context("Failed to enable raw mode")?;
        
        // Get terminal size
        let (width, height) = terminal::size().context("Failed to get terminal size")?;
        
        Ok(Self { width, height })
    }
    
    /// Get terminal width
    pub fn width(&self) -> u16 {
        self.width
    }
    
    /// Get terminal height
    pub fn height(&self) -> u16 {
        self.height
    }
    
    /// Clear the terminal screen
    pub fn clear(&mut self) -> Result<()> {
        execute!(io::stdout(), Clear(ClearType::All)).context("Failed to clear terminal")
    }
    
    /// Run the main event loop with a callback for each frame
    pub fn run_event_loop<F>(&mut self, frame_duration: Duration, mut callback: F) -> Result<()>
    where
        F: FnMut(&mut Self) -> Result<()>,
    {
        let mut last_frame = Instant::now();
        
        loop {
            // Handle resize events
            if event::poll(Duration::from_millis(0))? {
                if let Event::Resize(width, height) = event::read()? {
                    self.width = width;
                    self.height = height;
                }
            }
            
            // Check if it's time for a new frame
            let current = Instant::now();
            let elapsed = current.duration_since(last_frame);
            
            if elapsed >= frame_duration {
                // Clear screen for new frame
                self.clear()?;
                
                // Run the frame callback
                callback(self)?;
                
                // Flush stdout
                io::stdout().flush()?;
                
                // Update last frame time
                last_frame = current;
            }
            
            // Check for user input with a small timeout
            if event::poll(Duration::from_millis(10))? {
                if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                    match code {
                        // Quit on 'q' or Ctrl+C
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => break,
                        _ => {}
                    }
                }
            }
        }
        
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        // Disable raw mode
        let _ = terminal::disable_raw_mode();
        
        // Leave alternate screen and show cursor
        let _ = execute!(io::stdout(), LeaveAlternateScreen, Show);
    }
}
