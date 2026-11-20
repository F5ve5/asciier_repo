use crate::pref;

use image::{DynamicImage, ImageReader};

pub fn get_image(filename: &str) -> Result<DynamicImage, image::ImageError> {
    let path = std::path::Path::new(pref::PIC_FOLDER_NAME).join(filename);
    let img = ImageReader::open(path)?.decode()?;
    Ok(img)
}