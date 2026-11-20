use crate::{config, misc::{self,rgb_to_byteness,map_u8_to_x}, wincap};

use crossterm::{
    cursor::MoveTo, execute, queue, style::{Color, SetForegroundColor, Stylize}, terminal::{Clear, ClearType}
};
use image::{DynamicImage, EncodableLayout};

use std::io::{Stdout, stdout};
use std::fmt::Write;

#[inline]
pub fn print_rgba_frame(fd: wincap::FrameData, cc: &ConvertedConfig0){
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0)
    ).unwrap();

    let w: usize = fd.res_width as usize * 4;

    let mut current_column = 1;

    if cc.cd{
        while current_column * w  <= fd.frame_bytes.len(){
            for pixel in fd.frame_bytes[(current_column - 1) * w..current_column * w].chunks_exact(4).step_by(cc.xpc * 4){
                let b = misc::rgba_to_byteness(pixel[0], pixel[1], pixel[2], pixel[3]);
                print!("{}", cc.c[misc::map_u8_to_x(b, cc.c.len())])
            }
            println!();
            current_column += cc.ypc * 4;
        }
    }else{
        while current_column * w  <= fd.frame_bytes.len(){
            for pixel in fd.frame_bytes[(current_column - 1) * w..current_column * w].chunks_exact(4).step_by(cc.xpc * 4){
                let b = misc::rgba_to_byteness(pixel[0], pixel[1], pixel[2], pixel[3]);
                print!("{}", cc.c[misc::map_u8_to_x(b, cc.c.len())])
            }
            println!();
            current_column += cc.ypc * 4;
        }

    }
}

pub fn print_dynamic_image(di: &image::DynamicImage, config: &config::Config){
    if config.color{
        let rgba = di.to_rgba8();
        
        let s = rgba_to_colored_string(&config, rgba.as_bytes(), rgba.width() as usize);

        println!("{}",s);
    }else{
        let luma = di.to_luma8();

        let s = luma_to_bnw_string(&config, luma.as_bytes(), luma.width() as usize);

        println!("{}",s);
    }
}

////

//When printing characters into the console, it happens to be important to keep the actual console interactions to as few as possible, so making one large string to print
//into the console at once is the optimal approach
fn rgba_to_colored_string(c: &&config::Config, rgba_bytes: &[u8], width: usize) -> String{
    let mut s = String::new();

    let mut current_row = 1;
    let row_jump = (c.x_per_c * c.y_ratio) as usize;

    let chars: Vec<char> = c.b_str.chars().collect();
    let cl = chars.len();

    while current_row * width * 4 < rgba_bytes.len(){
        for x in rgba_bytes[(current_row - 1) * width * 4..current_row * width * 4].chunks_exact(4).step_by(c.x_per_c as usize){
            let char = chars[map_u8_to_x(rgb_to_byteness(x[0], x[1], x[2]),cl)];

            write!(
                &mut s,
                "\x1b[38;2;{};{};{}m{}",
                x[0],x[1],x[2],char
            ).unwrap();
        }
        write!(
            &mut s,
            "\n"
        ).unwrap();

        current_row += row_jump;
    }

    s.push_str("\x1b[0m");

    return s
}

fn luma_to_bnw_string(c: &&config::Config, luma_bytes: &[u8], width: usize) -> String{
    let mut s = String::new();

    let mut current_row = 1;
    let row_jump = (c.x_per_c * c.y_ratio) as usize;

    let chars: Vec<char> = c.b_str.chars().collect();
    let cl = chars.len();

    while current_row * width < luma_bytes.len(){
        for x in luma_bytes[(current_row - 1) * width..current_row * width].iter().step_by(c.x_per_c as usize){
            let char = chars[map_u8_to_x(*x,cl)];

            s.push(char);
        }
        write!(
            &mut s,
            "\n"
        ).unwrap();

        current_row += row_jump;
    }

    s.push_str("\x1b[0m");

    return s
}

////

pub trait FromConfig{
    fn from_config(c: config::Config) -> Self;
}

pub struct ConvertedConfig0{
    xpc: usize,
    ypc: usize,
    c: Vec<char>,
    cd: bool
}

impl FromConfig for ConvertedConfig0{
    fn from_config(c: config::Config) -> Self {
        Self { 
            xpc: c.x_per_c as usize,
            ypc: (c.y_ratio * c.x_per_c) as usize,
            c: c.b_str.chars().collect(),
            cd: c.color 
        }
    }
}