use crate::ui::component::{Component, Rect};
use anyhow::Result;
use std::collections::HashMap;

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct Layout {
    bounds: Rect,
    children: Vec<Box<dyn Component>>,
    orientation: Orientation,
    spacing: i32,
    padding: i32,
}

impl Layout {
    pub fn new(x: i32, y: i32, width: i32, height: i32, orientation: Orientation) -> Self {
        Self {
            bounds: Rect::new(x, y, width, height),
            children: Vec::new(),
            orientation,
            spacing: 5,
            padding: 5,
        }
    }

    pub fn set_spacing(&mut self, spacing: i32) {
        self.spacing = spacing;
        self.reflow();
    }

    pub fn set_padding(&mut self, padding: i32) {
        self.padding = padding;
        self.reflow();
    }

    pub fn add_child(&mut self, component: Box<dyn Component>) {
        self.children.push(component);
        self.reflow();
    }

    pub fn children(&self) -> &[Box<dyn Component>] {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut [Box<dyn Component>] {
        &mut self.children
    }

    fn reflow(&mut self) {
        let mut current_offset = self.padding;
        
        match self.orientation {
            Orientation::Horizontal => {
                // Calculate total width of fixed-width components
                let mut total_fixed_width = 0;
                let mut flex_components = 0;
                
                for child in &self.children {
                    let child_bounds = child.bounds();
                    if child_bounds.width > 0 {
                        total_fixed_width += child_bounds.width + self.spacing;
                    } else {
                        flex_components += 1;
                    }
                }
                
                // Calculate flex width
                let available_width = self.bounds.width - 2 * self.padding - 
                    (if !self.children.is_empty() { self.spacing * (self.children.len() as i32 - 1) } else { 0 });
                let flex_width = if flex_components > 0 {
                    (available_width - total_fixed_width) / flex_components
                } else {
                    0
                };
                
                // Position and size each component
                for child in &mut self.children {
                    let mut child_bounds = child.bounds().clone();
                    
                    // Set width if it's a flex component
                    if child_bounds.width <= 0 {
                        child_bounds.width = flex_width;
                    }
                    
                    // Set position
                    child_bounds.x = self.bounds.x + current_offset;
                    child_bounds.y = self.bounds.y + self.padding;
                    child_bounds.height = self.bounds.height - 2 * self.padding;
                    
                    child.set_position(child_bounds.x, child_bounds.y);
                    child.set_size(child_bounds.width, child_bounds.height);
                    
                    current_offset += child_bounds.width + self.spacing;
                }
            },
            
            Orientation::Vertical => {
                // Calculate total height of fixed-height components
                let mut total_fixed_height = 0;
                let mut flex_components = 0;
                
                for child in &self.children {
                    let child_bounds = child.bounds();
                    if child_bounds.height > 0 {
                        total_fixed_height += child_bounds.height + self.spacing;
                    } else {
                        flex_components += 1;
                    }
                }
                
                // Calculate flex height
                let available_height = self.bounds.height - 2 * self.padding - 
                    (if !self.children.is_empty() { self.spacing * (self.children.len() as i32 - 1) } else { 0 });
                let flex_height = if flex_components > 0 {
                    (available_height - total_fixed_height) / flex_components
                } else {
                    0
                };
                
                // Position and size each component
                for child in &mut self.children {
                    let mut child_bounds = child.bounds().clone();
                    
                    // Set height if it's a flex component
                    if child_bounds.height <= 0 {
                        child_bounds.height = flex_height;
                    }
                    
                    // Set position
                    child_bounds.x = self.bounds.x + self.padding;
                    child_bounds.y = self.bounds.y + current_offset;
                    child_bounds.width = self.bounds.width - 2 * self.padding;
                    
                    child.set_position(child_bounds.x, child_bounds.y);
                    child.set_size(child_bounds.width, child_bounds.height);
                    
                    current_offset += child_bounds.height + self.spacing;
                }
            }
        }
    }
}

impl Component for Layout {
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn set_position(&mut self, x: i32, y: i32) {
        let dx = x - self.bounds.x;
        let dy = y - self.bounds.y;
        
        self.bounds.x = x;
        self.bounds.y = y;
        
        // Update child positions
        for child in &mut self.children {
            let child_bounds = child.bounds();
            child.set_position(child_bounds.x + dx, child_bounds.y + dy);
        }
    }

    fn set_size(&mut self, width: i32, height: i32) {
        self.bounds.width = width;
        self.bounds.height = height;
        self.reflow();
    }

    fn render(&self, paint: &mut crate::display::Paint) -> Result<()> {
        // Render all children
        for child in &self.children {
            child.render(paint)?;
        }
        
        Ok(())
    }

    fn handle_tap(&mut self, x: i32, y: i32) -> bool {
        // Check if tap is within any child
        for child in &mut self.children {
            if child.handle_tap(x, y) {
                return true;
            }
        }
        
        // If no child handled it, check if it's in our bounds
        self.bounds.contains(x, y)
    }

    fn update(&mut self) -> Result<()> {
        // Update all children
        for child in &mut self.children {
            child.update()?;
        }
        
        Ok(())
    }
}