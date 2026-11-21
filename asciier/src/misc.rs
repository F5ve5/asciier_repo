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

pub fn delay_millis(millis: u64){
    std::thread::sleep(std::time::Duration::from_millis(millis));
}

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};
use std::io::{stdout, Write};

pub fn clear_console() -> std::io::Result<()> {
    let mut stdout = stdout();

    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0)
    )?;

    stdout.flush()
}

pub fn move_to_tl_console()-> std::io::Result<()> {
    let mut stdout = stdout();

    execute!(
        stdout,
        MoveTo(0, 0)
    )?;

    stdout.flush()
}