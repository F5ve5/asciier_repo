use crate::{config, misc::{self,rgb_to_byteness,map_u8_to_x}, wincap};

use crossterm::{
    cursor::MoveTo, execute, queue, style::{Color, SetForegroundColor, Stylize}, terminal::{Clear, ClearType}
};
use image::{DynamicImage, EncodableLayout};

use std::io::{Stdout, stdout};
use std::fmt::Write;

#[inline]
pub fn print_rgba_frame(fd: wincap::FrameData, c: &config::Config){
    let w: usize = fd.res_width as usize;
    let bytes = fd.frame_bytes;

    misc::move_to_tl_console().unwrap();

    if c.color{
        let s = rgba_to_colored_string(&c, bytes.as_slice(), w);

        println!("{}",s);
    }else{
        let s = rgba_to_bnw_string(&c, bytes.as_slice(), w);

        println!("{}",s);
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

#[inline]
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

#[inline]
fn rgba_to_bnw_string(c: &&config::Config, rgba_bytes: &[u8], width: usize) -> String{
    let mut s = String::new();

    let mut current_row = 1;
    let row_jump = (c.x_per_c * c.y_ratio) as usize;

    let chars: Vec<char> = c.b_str.chars().collect();
    let cl = chars.len();

    while current_row * width * 4 < rgba_bytes.len(){
        for x in rgba_bytes[(current_row - 1) * width * 4..current_row * width * 4].chunks_exact(4).step_by(c.x_per_c as usize){
            let char = chars[map_u8_to_x(x[0],cl)];

            s.push(char);
        }
        write!(
            &mut s,
            "\n"
        ).unwrap();

        current_row += row_jump;
    }

    return s
}

#[inline]
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