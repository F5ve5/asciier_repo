use crate::{config, misc};

use config::Config;

use std::io::{self, Write};
use std::time::Instant;

use windows_capture::capture::{Context, GraphicsCaptureApiHandler};
use windows_capture::encoder::{AudioSettingsBuilder, ContainerSettingsBuilder, VideoEncoder, VideoSettingsBuilder};
use windows_capture::frame::Frame;
use windows_capture::graphics_capture_api::InternalCaptureControl;
use windows_capture::graphics_capture_picker::{GraphicsCapturePicker, PickedGraphicsCaptureItem};
use windows_capture::settings::{
    ColorFormat, CursorCaptureSettings, DirtyRegionSettings, DrawBorderSettings, MinimumUpdateIntervalSettings,
    SecondaryWindowSettings, Settings,
};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::mpsc;
use std::thread;

struct StreamCap {
    start: Instant,
    channels: (Receiver<bool>,Sender<FrameData>)
}

impl GraphicsCaptureApiHandler for StreamCap {
    type Flags = (Receiver<bool>,Sender<FrameData>);
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        Ok(Self{
            start: Instant::now(),
            channels: ctx.flags
        })
    }

    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {

        if self.channels.0.try_recv().is_ok(){

            let mut fd = frame.buffer().unwrap();

            self.channels.1.send(FrameData {
                res_width: fd.width(),
                res_height: fd.height(),
                frame_bytes: fd.as_raw_buffer().to_vec() 
            }).unwrap();
        }

        Ok(())
    }

    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture session ended");
        Ok(())
    }
}

struct FrameData{
    res_width: u32,
    res_height: u32,
    frame_bytes: Vec<u8>
}

pub fn spawn_wincap(item: Option<PickedGraphicsCaptureItem>,config: Config){

    let (tx0,rx0): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let (tx1,rx1): (Sender<FrameData>, Receiver<FrameData>) = mpsc::channel();

    let Some(item) = item else {
        println!("No item selected");
        return;
    };

    let settings = Settings::new(
        item,
        CursorCaptureSettings::Default,
        DrawBorderSettings::Default,
        SecondaryWindowSettings::Default,
        MinimumUpdateIntervalSettings::Default,
        DirtyRegionSettings::Default,
        ColorFormat::Rgba8,
        (rx0,tx1),
    );

    thread::spawn(move||{
        let cc = ConvertedConfig0::from_config(config);
        loop{
            tx0.send(true).unwrap();

            let fd = rx1.recv().unwrap();

            print_rgba_frame(fd,&cc);
        }
    });

    StreamCap::start(settings).expect("Stream capture failed");
}

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};
use std::io::stdout;

#[inline]
fn print_rgba_frame(fd: FrameData, cc: &ConvertedConfig0){
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0)
    ).unwrap();

    let w: usize = fd.res_width as usize * 4;

    let mut current_column = 1;

    while current_column * w  <= fd.frame_bytes.len(){
        for pixel in fd.frame_bytes[(current_column - 1) * w..current_column * w].chunks_exact(4).step_by(cc.xpc * 4){
            let b = misc::rgba_to_byteness(pixel[0], pixel[1], pixel[2], pixel[3]);
            print!("{}", cc.c[misc::map_u8_to_x(b, cc.c.len())])
        }
        println!();
        current_column += cc.ypc * 4;
    }
}

struct ConvertedConfig0{
    xpc: usize,
    ypc: usize,
    c: Vec<char>
}

trait FromConfig{
    fn from_config(c: Config) -> Self;
}

impl FromConfig for ConvertedConfig0{
    fn from_config(c: Config) -> Self {
        Self { 
            xpc: c.x_per_c as usize,
            ypc: (c.y_ratio * c.x_per_c) as usize,
            c: c.b_str.chars().collect() 
        }
    }
}