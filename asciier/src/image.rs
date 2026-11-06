use crate::misc;
use crate::pref;

use image::{DynamicImage, ImageReader};
use misc::Resultt;

pub fn get_image() -> Resultt<DynamicImage> {
    let img = ImageReader::open("bilder/dd74ef994b804cd956c249c230fa25a7.jpg")?.decode()?;
    Ok(img)
}

pub fn process_luma8(image: &DynamicImage, brightness_str: &'static str){
    let luma = image.to_luma8();

    let x_jump = pref::xpixels_per_char as usize;
    let y_jump = (x_jump * pref::x_to_y_ratio as usize) as usize;


    let mut pixels: Vec<Vec<u8>> = vec![vec![0u8;(luma.width()/x_jump as u32) as usize];(luma.height()/y_jump as u32)as usize];

    for y in (0..luma.height()).step_by(y_jump).enumerate(){
        for x in (0..luma.width()).step_by(x_jump).enumerate(){
            pixels[y.0][x.0] = luma.get_pixel(x.1, y.1)[0];
        }
    }

    let mut c = brightness_str.chars();    
    for y in pixels.iter().enumerate(){
        for x in y.1.iter().enumerate(){
            let pixel_brightness = (pixels[y.0][x.0]/brightness_str.len() as u8) as usize;
            print!("{}", c.nth(pixel_brightness as usize).unwrap())
        }
        println!();
    }
}