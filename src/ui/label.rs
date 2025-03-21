use crate::display::{Paint, COLORED};
use crate::fonts::Font;
use crate::ui::component::{Component, Rect};
use anyhow::Result;

pub enum TextAlignment {
    Left,
    Center,
    Right,
}

pub struct Label {
    bounds: Rect,
    text: String,
    font: &'static Font,
    alignment: TextAlignment,
}

impl Label {
    pub fn new(x: i32, y: i32, width: i32, height: i32, text: &str, font: &'static Font) -> Self {
        Self {
            bounds: Rect::new(x, y, width, height),
            text: text.to_string(),
            font,
            alignment: TextAlignment::Left,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_alignment(&mut self, alignment: TextAlignment) {
        self.alignment = alignment;
    }

    pub fn alignment(&self) -> &TextAlignment {
        &self.alignment
    }
}

impl Component for Label {
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
        let char_width = self.font.width as i32;
        let text_width = char_width * self.text.len() as i32;
        
        // Calculate text position based on alignment
        let text_x = match self.alignment {
            TextAlignment::Left => self.bounds.x,
            TextAlignment::Center => self.bounds.x + (self.bounds.width - text_width) / 2,
            TextAlignment::Right => self.bounds.x + self.bounds.width - text_width,
        };
        
        // Vertically center the text
        let text_y = self.bounds.y + (self.bounds.height - self.font.height as i32) / 2;
        
        paint.draw_string_at(text_x, text_y, &self.text, self.font, COLORED);
        
        Ok(())
    }

    // Labels typically don't handle taps, so we'll keep the default implementation

    fn update(&mut self) -> Result<()> {
        // Nothing to update for a basic label
        Ok(())
    }
}