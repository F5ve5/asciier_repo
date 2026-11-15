mod image;
mod misc;
mod pref;
mod config;
mod files;
mod wincap;
mod args;
mod fax;

fn main() {


    let config = config::init();


    config::configure(config);
}