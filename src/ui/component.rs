use crate::display::Paint;
use anyhow::Result;
use downcast_rs::{Downcast, impl_downcast};

/// Defines the position and size of a UI component
#[derive(Debug, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

/// Base trait for all UI components
pub trait Component: Downcast {
    /// Get the bounding rectangle of the component
    fn bounds(&self) -> &Rect;
    
    /// Set the position of the component
    fn set_position(&mut self, x: i32, y: i32);
    
    /// Set the size of the component
    fn set_size(&mut self, width: i32, height: i32);
    
    /// Render the component to the given paint surface
    fn render(&self, paint: &mut Paint) -> Result<()>;
    
    /// Handle a tap/click event at the given coordinates
    /// Returns true if the event was handled
    fn handle_tap(&mut self, x: i32, y: i32) -> bool {
        self.bounds().contains(x, y)
    }
    
    /// Update the component state based on time or other factors
    fn update(&mut self) -> Result<()> {
        Ok(())
    }
}

// Implement downcasting for Component trait
impl_downcast!(Component);