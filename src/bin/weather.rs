use anyhow::Result;
use epaper_ui::display::{Epd4in2, Paint, COLORED, UNCOLORED};
use epaper_ui::fonts::FONT12;
use epaper_ui::ui::{Component, Label, Layout, Orientation, Rect, TextAlignment};
use std::thread;
use std::time::Duration;

// A simple component for drawing a horizontal divider line
struct DividerLine {
    bounds: Rect,
}

impl DividerLine {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            bounds: Rect::new(x, y, width, height),
        }
    }
}

impl Component for DividerLine {
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
        paint.draw_filled_rectangle(
            self.bounds.x, 
            self.bounds.y,
            self.bounds.x + self.bounds.width,
            self.bounds.y + self.bounds.height,
            COLORED,
        );
        Ok(())
    }
}

// A simple weather data struct (in a real app, this would come from an API)
struct WeatherData {
    location: String,
    temperature: i32, // in Celsius
    condition: String,
    humidity: u32,    // percentage
    wind_speed: f32,  // in km/h
}

impl WeatherData {
    fn sample_data() -> Self {
        Self {
            location: "San Francisco".to_string(),
            temperature: 18,
            condition: "Partly Cloudy".to_string(),
            humidity: 72,
            wind_speed: 12.5,
        }
    }
}

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    println!("Weather Display Demo");

    // Get sample weather data
    let weather = WeatherData::sample_data();

    // Initialize the display
    let mut epd = Epd4in2::new()?;
    epd.init()?;
    
    // Create a frame buffer for the display
    let width = epd.width();
    let height = epd.height();
    let mut paint = Paint::new(width, height);
    
    // Clear the buffer to white
    paint.clear(UNCOLORED);
    
    // Create a main layout
    let mut main_layout = Layout::new(0, 0, width as i32, height as i32, Orientation::Vertical);
    main_layout.set_padding(20);
    main_layout.set_spacing(15);
    
    // Add a header
    let mut header = Label::new(0, 0, 0, 50, "Weather Dashboard", &FONT12);
    header.set_alignment(TextAlignment::Center);
    main_layout.add_child(Box::new(header));
    
    // Add location
    let mut location = Label::new(0, 0, 0, 30, &format!("Location: {}", weather.location), &FONT12);
    location.set_alignment(TextAlignment::Left);
    main_layout.add_child(Box::new(location));
    
    // Create a layout for temperature and condition
    let mut temp_layout = Layout::new(0, 0, 0, 60, Orientation::Horizontal);
    temp_layout.set_spacing(20);
    
    // Add temperature
    let mut temperature = Label::new(
        0, 0, 150, 0, 
        &format!("{}°C", weather.temperature), 
        &FONT12
    );
    temperature.set_alignment(TextAlignment::Left);
    temp_layout.add_child(Box::new(temperature));
    
    // Add condition
    let mut condition = Label::new(
        0, 0, 0, 0,
        &weather.condition,
        &FONT12
    );
    condition.set_alignment(TextAlignment::Left);
    temp_layout.add_child(Box::new(condition));
    
    // Add the temperature layout to the main layout
    main_layout.add_child(Box::new(temp_layout));
    
    // Add a divider line
    let mut layout_for_line = Layout::new(0, 0, 0, 20, Orientation::Vertical);
    layout_for_line.add_child(Box::new(DividerLine::new(0, 0, 0, 2)));
    main_layout.add_child(Box::new(layout_for_line));
    
    // Add details layout
    let mut details_layout = Layout::new(0, 0, 0, 100, Orientation::Vertical);
    details_layout.set_spacing(10);
    
    // Add humidity
    let mut humidity = Label::new(
        0, 0, 0, 30,
        &format!("Humidity: {}%", weather.humidity),
        &FONT12
    );
    humidity.set_alignment(TextAlignment::Left);
    details_layout.add_child(Box::new(humidity));
    
    // Add wind speed
    let mut wind = Label::new(
        0, 0, 0, 30,
        &format!("Wind: {:.1} km/h", weather.wind_speed),
        &FONT12
    );
    wind.set_alignment(TextAlignment::Left);
    details_layout.add_child(Box::new(wind));
    
    // Add the details layout to the main layout
    main_layout.add_child(Box::new(details_layout));
    
    // Add a footer
    let mut footer = Label::new(
        0, 0, 0, 30,
        "Last updated: 2025-03-20 10:00",
        &FONT12
    );
    footer.set_alignment(TextAlignment::Center);
    main_layout.add_child(Box::new(footer));
    
    // Render the layout
    main_layout.render(&mut paint)?;
    
    // Draw a border around the screen
    paint.draw_rectangle(2, 2, width as i32 - 3, height as i32 - 3, COLORED);
    
    // Display the frame buffer
    epd.display_frame(paint.get_image())?;
    
    // Wait for a few seconds
    println!("Weather display updated. Sleeping for 10 seconds...");
    thread::sleep(Duration::from_secs(10));
    
    // Update the temperature (simulating a refresh)
    paint.clear(UNCOLORED);
    
    // Update the weather data
    let mut updated_weather = weather;
    updated_weather.temperature = 20;
    updated_weather.condition = "Sunny".to_string();
    
    // Update the component data
    if let Some(temp_layout) = main_layout.children_mut().get_mut(2) {
        if let Some(layout) = temp_layout.downcast_mut::<Layout>() {
            if let Some(temp) = layout.children_mut().get_mut(0) {
                if let Some(label) = temp.downcast_mut::<Label>() {
                    label.set_text(&format!("{}°C", updated_weather.temperature));
                }
            }
            if let Some(cond) = layout.children_mut().get_mut(1) {
                if let Some(label) = cond.downcast_mut::<Label>() {
                    label.set_text(&updated_weather.condition);
                }
            }
        }
    }
    
    // Update the footer
    if let Some(footer_component) = main_layout.children_mut().last_mut() {
        if let Some(label) = footer_component.downcast_mut::<Label>() {
            label.set_text("Last updated: 2025-03-20 10:10");
        }
    }
    
    // Re-render the layout
    main_layout.render(&mut paint)?;
    
    // Draw the border again
    paint.draw_rectangle(2, 2, width as i32 - 3, height as i32 - 3, COLORED);
    
    // Update the display
    epd.display_frame(paint.get_image())?;
    
    println!("Weather display updated with new data. Sleeping for 5 seconds...");
    thread::sleep(Duration::from_secs(5));
    
    // Put the display to sleep
    println!("Putting display to sleep");
    epd.sleep()?;
    
    Ok(())
}