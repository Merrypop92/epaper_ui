use anyhow::Result;
use std::cmp::{max, min};

// Constants for rotation
pub const ROTATE_0: u8 = 0;
pub const ROTATE_90: u8 = 1;
pub const ROTATE_180: u8 = 2;
pub const ROTATE_270: u8 = 3;

// Constants for color
pub const COLORED: bool = true;
pub const UNCOLORED: bool = false;

pub struct Paint {
    image: Vec<u8>,
    width: u32,
    height: u32,
    rotate: u8,
}

impl Paint {
    /// Create a new Paint instance with a buffer of given width and height
    pub fn new(width: u32, height: u32) -> Self {
        let buffer_size = (width * height / 8) as usize;
        let image = vec![0xFF; buffer_size]; // Initialize to white
        
        Self {
            image,
            width,
            height,
            rotate: ROTATE_0,
        }
    }

    /// Create a Paint instance with an existing buffer
    pub fn with_buffer(buffer: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            image: buffer,
            width,
            height,
            rotate: ROTATE_0,
        }
    }

    /// Clear the buffer to the specified color
    pub fn clear(&mut self, colored: bool) {
        let fill_value = if colored { 0x00 } else { 0xFF };
        for byte in self.image.iter_mut() {
            *byte = fill_value;
        }
    }

    /// Get the buffer width
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Get the buffer height
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Set the rotation value
    pub fn set_rotate(&mut self, rotate: u8) {
        self.rotate = rotate;
    }

    /// Get the rotation value
    pub fn get_rotate(&self) -> u8 {
        self.rotate
    }

    /// Get the image buffer
    pub fn get_image(&self) -> &[u8] {
        &self.image
    }

    /// Draw a pixel at absolute coordinates (ignoring rotation)
    fn draw_absolute_pixel(&mut self, x: i32, y: i32, colored: bool) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }

        // Calculate the byte position in the buffer
        let x_pos = x as u32;
        let y_pos = y as u32;
        let addr = (x_pos / 8 + y_pos * (self.width / 8)) as usize;

        if addr >= self.image.len() {
            return;
        }

        // Set the specific bit
        if colored {
            self.image[addr] &= !(0x80 >> (x_pos % 8));
        } else {
            self.image[addr] |= 0x80 >> (x_pos % 8);
        }
    }

    /// Draw a pixel at coordinates, considering rotation
    pub fn draw_pixel(&mut self, x: i32, y: i32, colored: bool) {
        let point = self.rotate_pixel(x, y);
        self.draw_absolute_pixel(point.0, point.1, colored);
    }

    /// Apply rotation to a point
    fn rotate_pixel(&self, x: i32, y: i32) -> (i32, i32) {
        let (width, height) = (self.width as i32, self.height as i32);
        
        match self.rotate {
            ROTATE_0 => (x, y),
            ROTATE_90 => (width - y - 1, x),
            ROTATE_180 => (width - x - 1, height - y - 1),
            ROTATE_270 => (y, height - x - 1),
            _ => (x, y), // Default to no rotation
        }
    }

    /// Draw a character at the specified position
    pub fn draw_char_at(&mut self, x: i32, y: i32, ascii_char: char, font: &crate::fonts::Font, colored: bool) {
        if !font.has_char(ascii_char) {
            return;
        }

        let char_offset = font.get_char_offset(ascii_char);
        let char_width = font.width;
        let char_height = font.height;

        for row in 0..char_height {
            let char_data = font.get_data_at(char_offset + row as usize);
            for col in 0..char_width {
                let pixel_colored = (char_data & (0x80 >> col)) != 0;
                if pixel_colored {
                    self.draw_pixel(x + col as i32, y + row as i32, colored);
                }
            }
        }
    }

    /// Draw a string at the specified position
    pub fn draw_string_at(&mut self, x: i32, y: i32, text: &str, font: &crate::fonts::Font, colored: bool) {
        let mut cursor_x = x;
        let char_width = font.width as i32;
        
        for c in text.chars() {
            self.draw_char_at(cursor_x, y, c, font, colored);
            cursor_x += char_width;
        }
    }

    /// Draw a line from (x0,y0) to (x1,y1)
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, colored: bool) {
        // Bresenham's line algorithm
        let mut steep = false;
        let mut x0_copy = x0;
        let mut y0_copy = y0;
        let mut x1_copy = x1;
        let mut y1_copy = y1;

        // If the line is steep, transpose the coordinates
        if (y1_copy - y0_copy).abs() > (x1_copy - x0_copy).abs() {
            std::mem::swap(&mut x0_copy, &mut y0_copy);
            std::mem::swap(&mut x1_copy, &mut y1_copy);
            steep = true;
        }

        // Make sure x0 < x1
        if x0_copy > x1_copy {
            std::mem::swap(&mut x0_copy, &mut x1_copy);
            std::mem::swap(&mut y0_copy, &mut y1_copy);
        }

        let dx = x1_copy - x0_copy;
        let dy = (y1_copy - y0_copy).abs();
        let mut err = dx / 2;
        let mut y_step: i32;
        let mut y = y0_copy;

        if y0_copy < y1_copy {
            y_step = 1;
        } else {
            y_step = -1;
        }

        for x in x0_copy..=x1_copy {
            if steep {
                self.draw_pixel(y, x, colored);
            } else {
                self.draw_pixel(x, y, colored);
            }
            err -= dy;
            if err < 0 {
                y += y_step;
                err += dx;
            }
        }
    }

    /// Draw a horizontal line
    pub fn draw_horizontal_line(&mut self, x: i32, y: i32, width: i32, colored: bool) {
        for i in 0..width {
            self.draw_pixel(x + i, y, colored);
        }
    }

    /// Draw a vertical line
    pub fn draw_vertical_line(&mut self, x: i32, y: i32, height: i32, colored: bool) {
        for i in 0..height {
            self.draw_pixel(x, y + i, colored);
        }
    }

    /// Draw a rectangle
    pub fn draw_rectangle(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, colored: bool) {
        let min_x = min(x0, x1);
        let max_x = max(x0, x1);
        let min_y = min(y0, y1);
        let max_y = max(y0, y1);

        self.draw_horizontal_line(min_x, min_y, max_x - min_x + 1, colored);
        self.draw_horizontal_line(min_x, max_y, max_x - min_x + 1, colored);
        self.draw_vertical_line(min_x, min_y, max_y - min_y + 1, colored);
        self.draw_vertical_line(max_x, min_y, max_y - min_y + 1, colored);
    }

    /// Draw a filled rectangle
    pub fn draw_filled_rectangle(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, colored: bool) {
        let min_x = min(x0, x1);
        let max_x = max(x0, x1);
        let min_y = min(y0, y1);
        let max_y = max(y0, y1);

        for y in min_y..=max_y {
            self.draw_horizontal_line(min_x, y, max_x - min_x + 1, colored);
        }
    }

    /// Draw a circle
    pub fn draw_circle(&mut self, x: i32, y: i32, radius: i32, colored: bool) {
        // Bresenham's circle algorithm
        let mut x_pos = -radius;
        let mut y_pos = 0;
        let mut err = 2 - 2 * radius;
        
        loop {
            self.draw_pixel(x - x_pos, y + y_pos, colored);
            self.draw_pixel(x + x_pos, y + y_pos, colored);
            self.draw_pixel(x + x_pos, y - y_pos, colored);
            self.draw_pixel(x - x_pos, y - y_pos, colored);
            
            let radius_err = err;
            if y_pos < 0 && radius_err <= y_pos {
                y_pos += 1;
                err += y_pos * 2 + 1;
            }
            if radius_err > x_pos || err > y_pos {
                x_pos += 1;
                err += x_pos * 2 + 1;
            }
            if x_pos > 0 {
                break;
            }
        }
    }

    /// Draw a filled circle
    pub fn draw_filled_circle(&mut self, x: i32, y: i32, radius: i32, colored: bool) {
        // Bresenham's circle algorithm with filling
        let mut x_pos = -radius;
        let mut y_pos = 0;
        let mut err = 2 - 2 * radius;
        
        while x_pos <= 0 {
            self.draw_horizontal_line(x + x_pos, y - y_pos, -2 * x_pos + 1, colored);
            self.draw_horizontal_line(x + x_pos, y + y_pos, -2 * x_pos + 1, colored);
            
            let radius_err = err;
            if y_pos < 0 && radius_err <= y_pos {
                y_pos += 1;
                err += y_pos * 2 + 1;
            }
            if radius_err > x_pos || err > y_pos {
                x_pos += 1;
                err += x_pos * 2 + 1;
            }
        }
    }
}