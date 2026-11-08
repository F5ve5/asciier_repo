use crate::config;
use crate::pref;

use std::fs::OpenOptions;
use config::Config;

pub fn check_pic_folder() -> bool {
    let folder = pref::PIC_FOLDER_NAME;

    if std::path::Path::new(folder).is_dir() {
        true
    } else {
        std::fs::create_dir_all(folder).unwrap();

        let image_path = std::path::Path::new(folder).join("rock_lee.jpg");

        std::fs::write(image_path, pref::ROCK_LEE).unwrap();

        false
    }
}

pub fn check_config() -> (Option<config::Config>,String) {

    let file_name = pref::CONFIG_NAME;

    match std::fs::read(file_name){

        Ok(config_se) => {
            let (config_de, _): (Config, usize) = match bincode::decode_from_slice::<Config, _>(
                    &config_se,
                    bincode::config::standard()
            ){
                Ok(x) => x,
                Err(_) => {
                    OpenOptions::new()
                        .write(true)
                        .read(false)
                        .create(true)
                        .open(file_name)
                        .expect("Failed to create config file");

                    return (None, String::from("Old config file corrupted, new one created"))
                }
            };

            return (Some(config_de), String::from("Config loaded"))
        },

        Err(_) => {
            OpenOptions::new()
                .write(true)
                .read(false)
                .create(true)
                .open(file_name)
                .expect("Failed to create config file");

            return (None, String::from("No config file, new one created"))
        }

    };
}

pub fn update_config(new_config: config::Config) -> std::io::Result<()>{
    Ok(())
}