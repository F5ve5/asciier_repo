use crate::misc;
use crate::config;
use crate::pref;

use image::{DynamicImage, ImageReader};

pub fn get_image(filename: &str) -> Result<DynamicImage, image::ImageError> {
    let path = std::path::Path::new(pref::PIC_FOLDER_NAME).join(filename);
    let img = ImageReader::open(path)?.decode()?;
    Ok(img)
}

pub fn print_dynamic_image(di: &DynamicImage, config: &config::Config){
    let luma = di.to_luma8();

    let x_jump = config.x_per_c as usize;
    let y_jump = x_jump * config.y_ratio as usize;

    let chars: Vec<char> = config.b_str.chars().collect();

    for y in (0..luma.height()).step_by(y_jump){
        for x in (0..luma.width()).step_by(x_jump){
            print!("{}",chars[misc::map_u8_to_x(luma.get_pixel(x,y)[0], chars.len())]);
        }
        println!();
    }
}