use crate::misc;
use crate::files;
use crate::image;
use crate::pref;
use crate::wincap;
use crate::fax;

use std::time::Duration;
use std::io;

#[derive(bincode::Decode,bincode::Encode,Debug)]
pub struct Config{
    pub image_dir: String,
    pub x_per_c: u16,
    pub y_ratio: u16,
    pub b_str: String,
}

pub fn init() -> Config{
    if !files::check_pic_folder(){
        fax::help();
    }

    let config;
    if let Some(c) = files::check_config(){
        config = c;
    }else{
        fax::help();

        config = Config{
            image_dir: String::from("x"),
            x_per_c: 5,
            y_ratio: 2,
            b_str: String::from(" ¨.,:-;~¤!£#@")
        }
    }

    config
}


pub fn configure(mut c: Config) {

    let mut i_buf = String::new();

    loop{
        println!("Enter 1 to ASCCIfy an image, press 2 for help with recording etc.");

        i_buf.clear();
        io::stdin().read_line(&mut i_buf).unwrap();

        let choice: u8 = match i_buf.trim().parse(){
            Ok(c) => c,
            Err(_) => {
                println!("Invalid input... Try again");
                continue;
            }
        };

        match choice{
            1 => {
                pick_image(&mut c);
                image_settings(&mut c);

                let di = match image::get_image(&c.image_dir){
                    Ok(di) => {di},
                    Err(e) => {
                        println!("Unable to process supposed image file {} : {:?}", c.image_dir, e);
                        continue;
                    }
                };
                image::print_dynamic_image(&di, &c);
            }
            2 => {
                break;
            }
            _ => {
                println!("Invalid input... Try again");
                continue;
            }
        }

        println!();
        match files::update_config(&c){
            Ok(_) => println!("Config updated successfully"),
            Err(e) => println!("Could not update config: {}",e)
        }
    }

}

fn pick_image(c: &mut Config) {

    let mut i_buf = String::new();

    println!();
    println!("Choose Image to ASCIIfy (Enter the number)");
    println!("Press Enter to leave as default ({:?})", c.image_dir);
    let fnames: Vec<String> = std::fs::read_dir(pref::PIC_FOLDER_NAME)
        .unwrap()
        .filter_map(|entry| {
            entry.ok()?.file_name().into_string().ok()
        })
        .collect();

    println!();
    for fname in fnames.iter().enumerate(){
        println!("{} -- {:?}", fname.0, fname.1);
    }

    loop{
        i_buf.clear();
        io::stdin().read_line(&mut i_buf).unwrap();

        if i_buf.trim().is_empty(){
            break;
        }

        let f: u16 = match i_buf.trim().parse(){
            Ok(f) => f,
            Err(_) => {
                println!("Invalid input... Try again");
                continue;
            }
        };

        if f >= fnames.len() as u16{
            println!("Invalid input... Try again");
            continue;
        }

        c.image_dir = fnames[f as usize].clone();
        break;
    }
}

pub fn image_settings(c: &mut Config){
    let mut i_buf = String::new();

    println!();
    println!("Set amount of horizontal pixels that are to be accounted for by each character");
    println!("Recommended to leave as default unless you want a stretched output");
    println!("Press Enter to leave as default ({})", c.x_per_c);
    loop{
        i_buf.clear();
        io::stdin().read_line(&mut i_buf).unwrap();

        if i_buf.trim().is_empty(){
            break;
        }

        let a: u16 = match i_buf.trim().parse(){
            Ok(a) => a,
            Err(_) => {
                println!("Invalid input... Try again");
                continue;
            }
        };

        c.x_per_c = a;
        break;
    }

    println!();
    println!("Set ratio between X and Y for accounted for pixels");
    println!("Press Enter to leave as default ({})", c.y_ratio);
    loop{
        i_buf.clear();
        io::stdin().read_line(&mut i_buf).unwrap();

        if i_buf.trim().is_empty(){
            break;
        }

        let r: u16 = match i_buf.trim().parse(){
            Ok(r) => r,
            Err(_) => {
                println!("Invalid input... Try again");
                continue;
            }
        };

        c.y_ratio = r;
        break;
    }

    println!();
    println!("Choose brightness string");
    println!("Press Enter to leave as default ({})", c.b_str);
    println!("Enter {} to write your own", pref::STRS.len());
    println!();
    for s in pref::STRS.iter().enumerate(){
        println!("{} -- {}", s.0, s.1);
    }
    loop{
        i_buf.clear();
        io::stdin().read_line(&mut i_buf).unwrap();

        if i_buf.trim().is_empty(){
            break;
        }

        let s: u16 = match i_buf.trim().parse(){
            Ok(s) => s,
            Err(_) => {
                println!("Invalid input... Try again");
                continue;
            }
        };

        if s < pref::STRS.len() as u16{
            c.b_str = String::from(pref::STRS[s as usize]);
        }else if s == pref::STRS.len() as u16{
            println!("The darkest pixels will be converted to the earliest characters so it should look something like [ .:;ooO0#@]");
            println!("3 characters minimum");
            loop{
                i_buf.clear();
                io::stdin().read_line(&mut i_buf).unwrap();

                let trimmed = i_buf.trim();

                if trimmed.chars().count() >= 3{
                    c.b_str = trimmed.to_string();
                    break;
                }else{
                    println!("Try again");
                    continue;
                }
            }
        }
        break;
    }
}