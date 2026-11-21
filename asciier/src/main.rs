mod image;
mod misc;
mod pref;
mod config;
mod files;
mod wincap;
mod args;
mod fax;
mod print;

fn main() {
    let args = args::retrieve();

    let config = config::init();

    config::configure(config,1);
}