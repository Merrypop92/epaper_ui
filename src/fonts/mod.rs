pub mod font12;

pub use font12::FONT12;

#[derive(Debug, Clone)]
pub struct Font {
    pub data: &'static [u8],
    pub width: u8,
    pub height: u8,
}

impl Font {
    pub fn has_char(&self, c: char) -> bool {
        let char_code = c as u32;
        // Assuming ASCII only for simplicity
        char_code >= 32 && char_code <= 126
    }
    
    pub fn get_char_offset(&self, c: char) -> usize {
        let char_code = c as u32;
        if !self.has_char(c) {
            return 0; // Return offset for space if character not found
        }
        
        // Calculate offset in the font data
        ((char_code - 32) * self.height as u32) as usize
    }
    
    pub fn get_data_at(&self, offset: usize) -> u8 {
        if offset >= self.data.len() {
            return 0;
        }
        self.data[offset]
    }
}