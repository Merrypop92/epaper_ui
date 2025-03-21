use anyhow::{anyhow, Result};
use rppal::gpio::{Gpio, Level, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::thread;
use std::time::Duration;

// Constants from epd4in2.h
pub const EPD_WIDTH: u32 = 800;
pub const EPD_HEIGHT: u32 = 480;

// Command definitions
const PANEL_SETTING: u8 = 0x00;
const POWER_SETTING: u8 = 0x01;
const POWER_OFF: u8 = 0x02;
const POWER_OFF_SEQUENCE_SETTING: u8 = 0x03;
const POWER_ON: u8 = 0x04;
const POWER_ON_MEASURE: u8 = 0x05;
const BOOSTER_SOFT_START: u8 = 0x06;
const DEEP_SLEEP: u8 = 0x07;
const DATA_START_TRANSMISSION_1: u8 = 0x10;
const DATA_STOP: u8 = 0x11;
const DISPLAY_REFRESH: u8 = 0x12;
const DATA_START_TRANSMISSION_2: u8 = 0x13;
const VCOM_AND_DATA_INTERVAL_SETTING: u8 = 0x50;
const RESOLUTION_SETTING: u8 = 0x61;
const VCM_DC_SETTING: u8 = 0x82;

// Pin definitions from epdif.h
const RST_PIN: u8 = 17;  // GPIO 17
const DC_PIN: u8 = 25;   // GPIO 25
const CS_PIN: u8 = 8;    // GPIO 8 (CE0)
const BUSY_PIN: u8 = 24; // GPIO 24

pub struct Epd4in2 {
    width: u32,
    height: u32,
    spi: Spi,
    reset_pin: OutputPin,
    dc_pin: OutputPin,
    busy_pin: rppal::gpio::InputPin,
}

// Implementation of the core functionality
impl Epd4in2 {
    pub fn new() -> Result<Self> {
        // Initialize SPI with correct settings
        let spi = Spi::new(
            Bus::Spi0,
            SlaveSelect::Ss0,
            10_000_000, // 10 MHz - you might need to adjust this based on stability
            Mode::Mode0, // SPI mode 0
        )?;

        // Initialize GPIO pins
        let gpio = Gpio::new()?;
        let reset_pin = gpio.get(RST_PIN)?.into_output();
        let dc_pin = gpio.get(DC_PIN)?.into_output();
        let busy_pin = gpio.get(BUSY_PIN)?.into_input();

        Ok(Self {
            width: EPD_WIDTH,
            height: EPD_HEIGHT,
            spi,
            reset_pin,
            dc_pin,
            busy_pin,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn init(&mut self) -> Result<()> {
        // Hardware reset
        self.reset();
        
        // Initial commands for display setup
        self.send_command(POWER_SETTING)?;
        self.send_data(&[0x03, 0x00, 0x2B, 0x2B, 0x03])?;

        self.send_command(BOOSTER_SOFT_START)?;
        self.send_data(&[0x17, 0x17, 0x17])?;

        self.send_command(POWER_ON)?;
        self.wait_until_idle();

        self.send_command(PANEL_SETTING)?;
        self.send_data(&[0xBF, 0x0D])?;

        self.send_command(RESOLUTION_SETTING)?;
        self.send_data(&[
            (EPD_WIDTH >> 8) as u8, 
            (EPD_WIDTH & 0xFF) as u8,
            (EPD_HEIGHT >> 8) as u8, 
            (EPD_HEIGHT & 0xFF) as u8,
        ])?;

        self.send_command(VCM_DC_SETTING)?;
        self.send_data(&[0x28])?;

        self.send_command(VCOM_AND_DATA_INTERVAL_SETTING)?;
        self.send_data(&[0x97])?;

        self.set_lut()?;

        Ok(())
    }

    // Reset the display
    pub fn reset(&mut self) {
        self.reset_pin.write(Level::High);
        thread::sleep(Duration::from_millis(200));
        self.reset_pin.write(Level::Low);
        thread::sleep(Duration::from_millis(10));
        self.reset_pin.write(Level::High);
        thread::sleep(Duration::from_millis(200));
    }

    // Wait until the busy pin is released
    pub fn wait_until_idle(&self) {
        log::debug!("Waiting for display to be ready...");
        while self.busy_pin.read() == Level::Low {
            thread::sleep(Duration::from_millis(100));
        }
        log::debug!("Display is ready.");
    }

    // Send a command to the display
    pub fn send_command(&mut self, command: u8) -> Result<()> {
        self.dc_pin.write(Level::Low);
        self.spi.write(&[command])?;
        Ok(())
    }

    // Send data to the display
    pub fn send_data(&mut self, data: &[u8]) -> Result<()> {
        self.dc_pin.write(Level::High);
        self.spi.write(data)?;
        Ok(())
    }

    // Display a frame from the buffer
    pub fn display_frame(&mut self, frame_buffer: &[u8]) -> Result<()> {
        if frame_buffer.len() != (self.width * self.height / 8) as usize {
            return Err(anyhow!("Frame buffer size mismatch"));
        }

        let buffer_size = 4000; // Break into chunks for sending
        
        self.send_command(DATA_START_TRANSMISSION_1)?;
        
        for chunk in frame_buffer.chunks(buffer_size) {
            self.send_data(chunk)?;
        }

        self.send_command(DISPLAY_REFRESH)?;
        self.wait_until_idle();

        Ok(())
    }

    // Put display to sleep to save power
    pub fn sleep(&mut self) -> Result<()> {
        self.send_command(POWER_OFF)?;
        self.wait_until_idle();
        self.send_command(DEEP_SLEEP)?;
        self.send_data(&[0xA5])?;
        
        Ok(())
    }

    // Set the lookup table for waveform
    fn set_lut(&mut self) -> Result<()> {
        // You would implement the LUT (Look-Up Table) setting here
        // This is specific to the display controller and waveform
        // For now, we'll just return OK
        Ok(())
    }
}