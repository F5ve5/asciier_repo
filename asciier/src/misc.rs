use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use std::io::{self, Write};

pub type Errorr = Box<dyn std::error::Error>;
pub type Resultt<T> = std::result::Result<T, Errorr>;

pub fn map_u8_to_x(u8:u8,x:usize) -> usize{
    (u8 as usize * x/256)
}

pub fn rgba_to_byteness(r: u8, g: u8, b: u8, a: u8) -> u8 {
    let brightness =
        0.2126 * r as f32 +
        0.7152 * g as f32 +
        0.0722 * b as f32;

    (brightness * (a as f32 / 255.0)).round() as u8
}

pub fn rgb_to_byteness(r: u8, g: u8, b: u8) -> u8 {
    (
        0.2126 * r as f32 +
        0.7152 * g as f32 +
        0.0722 * b as f32
    ) as u8
}