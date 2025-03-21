use crate::display::{Paint, COLORED, UNCOLORED};
use crate::fonts::Font;
use crate::ui::component::{Component, Rect};
use anyhow::Result;
use std::sync::{Arc, Mutex};

pub type ButtonCallback = Arc<Mutex<dyn FnMut() -> Result<()> + Send>>;

pub struct Button {
    bounds: Rect,
    label: String,
    font: &'static Font,
    is_pressed: bool,
    on_click: Option<ButtonCallback>,
    is_enabled: bool,
}

impl Button {
    pub fn new(x: i32, y: i32, width: i32, height: i32, label: &str, font: &'static Font) -> Self {
        Self {
            bounds: Rect::new(x, y, width, height),
            label: label.to_string(),
            font,
            is_pressed: false,
            on_click: None,
            is_enabled: true,
        }
    }

    pub fn set_on_click<F>(&mut self, callback: F)
    where
        F: FnMut() -> Result<()> + Send + 'static,
    {
        self.on_click = Some(Arc::new(Mutex::new(callback)));
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

impl Component for Button {
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.bounds.x = x;
        self.bounds.y = y;
    }

    fn set_size(&mut self, width: i32, height: i32) {
        self.bounds.width = width;
        self.bounds.height = height;
    }

    fn render(&self, paint: &mut Paint) -> Result<()> {
        // Draw button background (filled rectangle)
        let bg_color = if self.is_enabled {
            if self.is_pressed {
                COLORED // Inverse when pressed
            } else {
                UNCOLORED
            }
        } else {
            UNCOLORED // Disabled state
        };

        paint.draw_filled_rectangle(
            self.bounds.x,
            self.bounds.y,
            self.bounds.x + self.bounds.width - 1,
            self.bounds.y + self.bounds.height - 1,
            bg_color,
        );

        // Draw button border
        paint.draw_rectangle(
            self.bounds.x,
            self.bounds.y,
            self.bounds.x + self.bounds.width - 1,
            self.bounds.y + self.bounds.height - 1,
            COLORED,
        );

        // Draw button text
        let text_color = if self.is_enabled {
            if self.is_pressed {
                UNCOLORED // Inverse when pressed
            } else {
                COLORED
            }
        } else {
            // Gray effect for disabled state (use pattern)
            COLORED
        };

        // Center the text
        let char_width = self.font.width as i32;
        let char_height = self.font.height as i32;
        let text_width = char_width * self.label.len() as i32;
        let text_x = self.bounds.x + (self.bounds.width - text_width) / 2;
        let text_y = self.bounds.y + (self.bounds.height - char_height) / 2;

        paint.draw_string_at(text_x, text_y, &self.label, self.font, text_color);

        Ok(())
    }

    fn handle_tap(&mut self, x: i32, y: i32) -> bool {
        if !self.is_enabled || !self.bounds.contains(x, y) {
            return false;
        }

        self.is_pressed = true;
        
        // Trigger the callback
        if let Some(ref callback) = self.on_click {
            if let Ok(mut cb) = callback.lock() {
                // Ignore errors from the callback for now
                let _ = cb();
            }
        }
        
        // Reset pressed state after a brief delay (would be handled by the UI loop)
        self.is_pressed = false;
        
        true
    }

    fn update(&mut self) -> Result<()> {
        // Nothing to update for a basic button
        Ok(())
    }
}