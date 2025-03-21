use anyhow::Result;
use epaper_ui::display::{Epd4in2, Paint, COLORED, UNCOLORED};
use epaper_ui::fonts::FONT12;
use epaper_ui::ui::{Button, Component, Label, Layout, Orientation, TextAlignment};
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    println!("Hello, E-Paper World!");

    // Initialize the display
    let mut epd = Epd4in2::new()?;
    epd.init()?;
    
    // Create a frame buffer for the display
    let width = epd.width();
    let height = epd.height();
    let mut paint = Paint::new(width, height);
    
    // Clear the buffer to white
    paint.clear(UNCOLORED);
    
    // Create a hello world message
    let mut layout = Layout::new(0, 0, width as i32, height as i32, Orientation::Vertical);
    layout.set_padding(20);
    layout.set_spacing(20);
    
    // Add a title
    let mut title = Label::new(0, 0, 0, 40, "Hello, E-Paper World!", &FONT12);
    title.set_alignment(TextAlignment::Center);
    layout.add_child(Box::new(title));
    
    // Add some text
    let text = "This is a simple example of the e-paper UI framework.";
    let mut description = Label::new(0, 0, 0, 30, text, &FONT12);
    description.set_alignment(TextAlignment::Center);
    layout.add_child(Box::new(description));
    
    // Add a button
    let mut button = Button::new(0, 0, 150, 50, "Press Me!", &FONT12);
    button.set_on_click(|| {
        println!("Button pressed!");
        Ok(())
    });
    layout.add_child(Box::new(button));
    
    // Render the layout
    layout.render(&mut paint)?;
    
    // Draw a border around the screen
    paint.draw_rectangle(2, 2, width as i32 - 3, height as i32 - 3, COLORED);
    
    // Display the frame buffer
    epd.display_frame(paint.get_image())?;
    
    // Wait for a few seconds
    println!("Display updated. Sleeping for 5 seconds...");
    thread::sleep(Duration::from_secs(5));
    
    // Put the display to sleep
    println!("Putting display to sleep");
    epd.sleep()?;
    
    Ok(())
}