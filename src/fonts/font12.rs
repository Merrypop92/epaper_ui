use super::Font;

// Font data based on Font12.c
pub static FONT12_DATA: &[u8] = &[
    // @0 ' ' (7 pixels wide)
    0x00, //        
    0x00, //        
    0x00, //        
    0x00, //        
    0x00, //        
    0x00, //        
    0x00, //        
    0x00, //        
    0x00, //        
    0x00, //        
    0x00, //        
    0x00, //        

    // @12 '!' (7 pixels wide)
    0x00, //        
    0x10, //    #   
    0x10, //    #   
    0x10, //    #   
    0x10, //    #   
    0x10, //    #   
    0x00, //        
    0x00, //        
    0x10, //    #   
    0x00, //        
    0x00, //        
    0x00, //        
    
    // Additional font data would be included here
    // For brevity, just including the first few characters
    // You would include the complete font table from font12.c
];

// Create a static reference to the font
pub static FONT12: Font = Font {
    data: FONT12_DATA,
    width: 7,
    height: 12,
};