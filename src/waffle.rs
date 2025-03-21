use std::f64::consts::PI;

/// Size options for the waffle
#[derive(Debug, Clone, Copy)]
pub enum Size {
    Small,
    Medium,
    Large,
}

/// The 3D point in space
#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    
    /// Rotate the point around the y-axis
    pub fn rotate_y(&mut self, angle: f64) {
        let old_x = self.x;
        let old_z = self.z;
        
        self.x = old_x * angle.cos() - old_z * angle.sin();
        self.z = old_x * angle.sin() + old_z * angle.cos();
    }
    
    /// Rotate the point around the x-axis
    pub fn rotate_x(&mut self, angle: f64) {
        let old_y = self.y;
        let old_z = self.z;
        
        self.y = old_y * angle.cos() - old_z * angle.sin();
        self.z = old_y * angle.sin() + old_z * angle.cos();
    }
    
    /// Rotate the point around the z-axis
    pub fn rotate_z(&mut self, angle: f64) {
        let old_x = self.x;
        let old_y = self.y;
        
        self.x = old_x * angle.cos() - old_y * angle.sin();
        self.y = old_x * angle.sin() + old_y * angle.cos();
    }
}

/// Waffle grid cell
#[derive(Debug, Clone)]
pub struct WaffleCell {
    pub point: Point3D,
    pub depth: f64,
    pub is_edge: bool,
}

/// The 3D waffle model
pub struct Waffle {
    cells: Vec<WaffleCell>,
    size: Size,
    rotation_x: f64,
    rotation_y: f64,
    rotation_z: f64,
    original_points: Vec<Point3D>,
}

impl Waffle {
    pub fn new(size: Size) -> Self {
        let (width, height, depth, grid_size) = match size {
            Size::Small => (5.0, 5.0, 1.0, 10),
            Size::Medium => (8.0, 8.0, 1.5, 16),
            Size::Large => (12.0, 12.0, 2.0, 24),
        };
        
        // Create the basic waffle grid
        let mut cells = Vec::new();
        let mut original_points = Vec::new();
        
        // Create top and bottom grids
        for z in [-depth / 2.0, depth / 2.0].iter() {
            for y in (0..grid_size).map(|i| height * (i as f64 / grid_size as f64 - 0.5)) {
                for x in (0..grid_size).map(|i| width * (i as f64 / grid_size as f64 - 0.5)) {
                    let is_edge = x.abs() >= (width / 2.0) - 0.1 || 
                                  y.abs() >= (height / 2.0) - 0.1;
                    
                    let point = Point3D::new(x, y, *z);
                    original_points.push(point);
                    
                    cells.push(WaffleCell {
                        point,
                        depth: point.z,
                        is_edge,
                    });
                }
            }
        }
        
        // Create the vertical connections between top and bottom
        for x in [-(width / 2.0), width / 2.0].iter() {
            for y in [-(height / 2.0), height / 2.0].iter() {
                for z in (0..5).map(|i| depth * (i as f64 / 4.0 - 0.5)) {
                    let point = Point3D::new(*x, *y, z);
                    original_points.push(point);
                    
                    cells.push(WaffleCell {
                        point,
                        depth: point.z,
                        is_edge: true,
                    });
                }
            }
        }
        
        // Create the waffle grid lines
        let grid_step = width / 4.0;
        for i in 1..4 {
            let pos = i as f64 * grid_step - width / 2.0;
            
            // Horizontal lines
            for y in [-height / 2.0, height / 2.0].iter() {
                for z in (0..5).map(|i| depth * (i as f64 / 4.0 - 0.5)) {
                    let point = Point3D::new(pos, *y, z);
                    original_points.push(point);
                    
                    cells.push(WaffleCell {
                        point,
                        depth: point.z,
                        is_edge: true,
                    });
                }
            }
            
            // Vertical lines
            for x in [-width / 2.0, width / 2.0].iter() {
                for z in (0..5).map(|i| depth * (i as f64 / 4.0 - 0.5)) {
                    let point = Point3D::new(*x, pos, z);
                    original_points.push(point);
                    
                    cells.push(WaffleCell {
                        point,
                        depth: point.z,
                        is_edge: true,
                    });
                }
            }
        }
        
        Self {
            cells,
            size,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            original_points,
        }
    }
    
    /// Get a reference to all waffle cells
    pub fn cells(&self) -> &[WaffleCell] {
        &self.cells
    }
    
    /// Get the current size
    pub fn size(&self) -> Size {
        self.size
    }
    
    /// Set the rotation angles
    pub fn set_rotation(&mut self, x: f64, y: f64, z: f64) {
        self.rotation_x = x;
        self.rotation_y = y;
        self.rotation_z = z;
        self.update_points();
    }
    
    /// Get current rotation values
    pub fn rotation(&self) -> (f64, f64, f64) {
        (self.rotation_x, self.rotation_y, self.rotation_z)
    }
    
    /// Update all points based on current rotation
    fn update_points(&mut self) {
        for (i, original) in self.original_points.iter().enumerate() {
            let mut point = *original;
            
            // Apply rotations
            point.rotate_x(self.rotation_x);
            point.rotate_y(self.rotation_y);
            point.rotate_z(self.rotation_z);
            
            // Update the cell
            self.cells[i].point = point;
            self.cells[i].depth = point.z;
        }
        
        // Sort cells by z-value for proper rendering (back-to-front)
        self.cells.sort_by(|a, b| b.depth.partial_cmp(&a.depth).unwrap_or(std::cmp::Ordering::Equal));
    }
    
    /// Update waffle rotation by increment
    pub fn rotate(&mut self, dx: f64, dy: f64, dz: f64) {
        self.rotation_x = (self.rotation_x + dx) % (2.0 * PI);
        self.rotation_y = (self.rotation_y + dy) % (2.0 * PI);
        self.rotation_z = (self.rotation_z + dz) % (2.0 * PI);
        self.update_points();
    }
}
