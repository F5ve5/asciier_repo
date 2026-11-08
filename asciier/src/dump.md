    let mut config = match files::check_config(){
        (Some(c),msg) => {println!("{}",msg); c},
        (None, msg) => {println!("{}",msg);
            Config{
                image_dir: String::from("lalala"),
                x_per_c: 6,
                y_ratio: 2,
                b_str: String::from(" .,-*ow9@")
            }
        }
    };