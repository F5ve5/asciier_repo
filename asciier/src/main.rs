mod image;
mod misc;
mod pref;

fn main() {
    match image::get_image(){
        Ok(di) => image::process_luma8(&di,pref::str0),
        Err(b) => println!("Error: {:?}",b)
    }
}
