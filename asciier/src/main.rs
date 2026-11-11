mod image;
mod misc;
mod pref;
mod config;
mod files;

fn main() {
    config::init();

    config::configure();
}
