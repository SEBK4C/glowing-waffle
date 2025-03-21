use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, SetForegroundColor},
};
use std::io;

use crate::{terminal::Terminal, waffle::{Waffle, Point3D}};

/// Available color modes for rendering
#[derive(Debug, Clone, Copy)]
pub enum ColorMode {
    Normal,
    Rainbow,
    Fire,
}

/// ASCII renderer for the waffle model
pub struct Renderer {
    color_mode: ColorMode,
    // Character palette from sparse to dense
    char_palette: Vec<char>,
}

impl Renderer {
    pub fn new(color_mode: ColorMode) -> Self {
        // Character palette from sparse to dense for depth representation
        let char_palette = vec![' ', '.', ':', ';', '!', '*', 'o', '&', '%', '#', '@'];
        
        Self { color_mode, char_palette }
    }
    
    /// Change the color mode
    pub fn set_color_mode(&mut self, mode: ColorMode) {
        self.color_mode = mode;
    }
    
    /// Get a character based on depth value
    fn get_char_for_depth(&self, depth: f64, is_edge: bool) -> char {
        if is_edge {
            return '#';
        }
        
        // Normalize depth to 0-1 range
        let depth_norm = (depth + 2.0) / 4.0;
        let depth_norm = depth_norm.clamp(0.0, 0.99);
        
        // Map to character palette
        let char_index = (depth_norm * self.char_palette.len() as f64) as usize;
        self.char_palette[char_index]
    }
    
    /// Get a color based on depth and position
    fn get_color(&self, point: &Point3D, depth: f64, time: f64) -> Color {
        match self.color_mode {
            ColorMode::Normal => {
                // Simple grayscale based on depth
                let brightness = ((depth + 2.0) / 4.0 * 200.0 + 55.0) as u8;
                Color::Rgb { r: brightness, g: brightness, b: brightness }
            },
            ColorMode::Rainbow => {
                // Rainbow pattern that shifts with time
                let hue = (point.x.atan2(point.y) + time) % (2.0 * std::f64::consts::PI);
                let hue_norm = hue / (2.0 * std::f64::consts::PI);
                
                // Map hue to RGB
                let brightness = ((depth + 2.0) / 4.0 * 155.0 + 100.0) as u8;
                
                if hue_norm < 0.167 {
                    Color::Rgb { r: brightness, g: (brightness as f64 * (hue_norm / 0.167)).round() as u8, b: 0 }
                } else if hue_norm < 0.334 {
                    Color::Rgb { r: (brightness as f64 * (1.0 - (hue_norm - 0.167) / 0.167)).round() as u8, g: brightness, b: 0 }
                } else if hue_norm < 0.5 {
                    Color::Rgb { r: 0, g: brightness, b: (brightness as f64 * ((hue_norm - 0.334) / 0.166)).round() as u8 }
                } else if hue_norm < 0.667 {
                    Color::Rgb { r: 0, g: (brightness as f64 * (1.0 - (hue_norm - 0.5) / 0.167)).round() as u8, b: brightness }
                } else if hue_norm < 0.834 {
                    Color::Rgb { r: (brightness as f64 * ((hue_norm - 0.667) / 0.167)).round() as u8, g: 0, b: brightness }
                } else {
                    Color::Rgb { r: brightness, g: 0, b: (brightness as f64 * (1.0 - (hue_norm - 0.834) / 0.166)).round() as u8 }
                }
            },
            ColorMode::Fire => {
                // Fire effect
                let dist = (point.x.powi(2) + point.y.powi(2)).sqrt();
                let glow = (dist.sin() * 0.5 + 0.5).powi(2) * ((time * 3.0).sin() * 0.3 + 0.7);
                
                let r = (255.0 * glow) as u8;
                let g = (140.0 * glow * glow) as u8;
                let b = (40.0 * glow * glow * glow) as u8;
                
                Color::Rgb { r, g, b }
            }
        }
    }
    
    /// Render the waffle model
    pub fn render(&mut self, term: &mut Terminal, waffle: &Waffle) -> Result<()> {
        let width = term.width() as f64;
        let height = term.height() as f64;
        
        // Static time for now (will be updated by animation)
        let time = 0.0;
        
        // Map of characters to render
        let mut char_map = vec![vec![' '; term.width() as usize]; term.height() as usize];
        let mut color_map = vec![vec![Color::White; term.width() as usize]; term.height() as usize];
        let mut depth_map = vec![vec![-1000.0; term.width() as usize]; term.height() as usize];
        
        // Project 3D points to 2D screen coordinates
        for cell in waffle.cells() {
            // Simple perspective projection
            let z_factor = (cell.point.z + 5.0) / 10.0; // Avoid division by zero
            let screen_x = width / 2.0 + cell.point.x * 5.0 / z_factor;
            let screen_y = height / 2.0 + cell.point.y * 2.5 / z_factor;
            
            // Check if the point is within screen bounds
            if screen_x >= 0.0 && screen_x < width && screen_y >= 0.0 && screen_y < height {
                let x = screen_x as usize;
                let y = screen_y as usize;
                
                // Only draw the point if it's closer than what's already there (z-buffer)
                if cell.depth > depth_map[y][x] {
                    depth_map[y][x] = cell.depth;
                    char_map[y][x] = self.get_char_for_depth(cell.depth, cell.is_edge);
                    color_map[y][x] = self.get_color(&cell.point, cell.depth, time);
                }
            }
        }
        
        // Render the character map to the terminal
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        
        for y in 0..term.height() as usize {
            for x in 0..term.width() as usize {
                if char_map[y][x] != ' ' {
                    execute!(
                        stdout,
                        MoveTo(x as u16, y as u16),
                        SetForegroundColor(color_map[y][x]),
                        Print(char_map[y][x])
                    )?;
                }
            }
        }
        
        Ok(())
    }
}
