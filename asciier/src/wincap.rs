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

pub fn spawn_wincap(item: Option<PickedGraphicsCaptureItem>){

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

    StreamCap::start(settings).expect("Stream capture failed");

}
