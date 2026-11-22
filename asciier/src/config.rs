use windows_capture::graphics_capture_picker::PickedGraphicsCaptureItem;

use crate::misc;
use crate::files;
use crate::image;
use crate::pref;
use crate::wincap;
use crate::fax;
use crate::print;
use crate::input;

use std::io;

#[derive(bincode::Decode,bincode::Encode,Debug)]
pub struct Config{
    pub image_dir: String,
    pub x_per_c: u16,
    pub y_ratio: u16,
    pub b_str: String,
    pub color: bool
}

pub fn init() -> Config{
    let mut help = false;

    if !files::check_pic_folder(){
        help = true;
    }

    let config;
    if let Some(c) = files::check_config(){
        config = c;
    }else{
        help = true;

        config = Config{
            image_dir: String::from("x"),
            x_per_c: 5,
            y_ratio: 2,
            b_str: String::from(" ¨.,:-;~¤!£#@"),
            color: false
        }
    }

    if help{
        fax::help();
    }

    config
}


pub fn configure(mut c: Config, args: u8) {

    if args == 1{
        let mut item: Option<PickedGraphicsCaptureItem> = None;
        if let Some(i) = windows_capture::graphics_capture_picker::GraphicsCapturePicker::pick_item().expect("Failed to pick item"){
            item = Some(i);
        }else{
            panic!("item picker failed")
        }

        settings(&mut c);

        wincap::spawn_wincap(item,c);

        return
    }

    let mut i_buf = String::new();
    loop{
        println!("Enter 1 to ASCCIfy an image, Enter 2 for help with streaming etc. Enter 3 to close");

        let i = input::enter_u16(1, 3, false);

        match i{
            1 => {
                pick_image(&mut c);
                settings(&mut c);

                let di = match image::get_image(&c.image_dir){
                    Ok(di) => {di},
                    Err(e) => {
                        println!("Unable to process supposed image file {} : {:?}", c.image_dir, e);
                        continue;
                    }
                };
                print::print_dynamic_image(&di, &c);
            }
            2 => {
                fax::help();
                continue;
            }
            3 => {
                return;
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
    println!();
    println!("Choose Image to ASCIIfy");
    println!("Press R to leave as default ({:?})", c.image_dir);

    let fnames: Vec<String> = std::fs::read_dir(pref::PIC_FOLDER_NAME)
        .unwrap()
        .filter_map(|entry| {
            entry.ok()?.file_name().into_string().ok()
        })
        .collect();
    let fname_refs: Vec<&str> = fnames.iter()
        .map(|s| s.as_str())
        .collect();
    let index = input::choose_string(&fname_refs).unwrap();
    println!();

    if index != usize::MAX{
        c.image_dir = fnames[index].clone();
    }
}

pub fn settings(c: &mut Config){
    println!();
    println!("Enter amount of horizontal pixels that are to be accounted for by each character");
    println!("Press Enter to leave as default ({})", c.x_per_c);
    if let i @ 1.. = input::enter_u16(1, 1000, true) {
        c.x_per_c = i;
    }

    println!();
    println!("Enter ratio between X and Y for accounted-for pixels");
    println!("Recommended to leave as default unless you want a stretched output");
    println!("Press Enter to leave as default ({})", c.y_ratio);
    if let i @ 1.. = input::enter_u16(1, 1000, true) {
        c.y_ratio = i;
    }

    println!();
    println!("Choose brightness string");
    println!("Press R to leave as default ({})", c.b_str);
    if let Ok(i) = input::choose_string(&pref::STRS) {
        if i == pref::STRS.len() - 1{
            println!();
            println!("The darkest pixels will be converted to the earliest characters so it should look something like [ .:;ooO0#@]");
            println!("3 characters minimum");
            c.b_str = input::enter_str(3, None, false).unwrap();
        }
        else if i < pref::STRS.len(){
            c.b_str = String::from(pref::STRS[i]);
        }
    }
    println!();

    println!();
    println!("Colored or not?");
    println!("Press R to leave as default ({})", if c.color {"yup"}else{"nuh uh"});
    let yn = vec!["yea","nah"];
    match input::choose_string(&yn).unwrap(){
        0 => c.color = true,
        1 => c.color = false,
        _ => {}
    }
}