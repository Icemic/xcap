use std::sync::{Condvar, Mutex};

use crate::{XCapResult, platform::impl_video_recorder::ImplVideoRecorder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameFormat {
    RGB = 1,
    BGR = 2,
    ARGB = 3,
    BGRA = 4,
    RGBx = 5,
    BGRx = 6,
    RGBA = 7,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub raw: Vec<u8>,
    pub format: FrameFormat,
}

impl Frame {
    pub fn new(width: u32, height: u32, raw: Vec<u8>, format: FrameFormat) -> Self {
        Self {
            width,
            height,
            raw,
            format,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorMode {
    Hidden = 1,
    Show = 2,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct RecorderWaker {
    parking: Mutex<bool>,
    condvar: Condvar,
}

impl RecorderWaker {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            parking: Mutex::new(true),
            condvar: Condvar::new(),
        }
    }
    #[allow(dead_code)]
    pub fn wake(&self) -> XCapResult<()> {
        let mut parking = self.parking.lock()?;
        *parking = false;
        self.condvar.notify_one();

        Ok(())
    }
    #[allow(dead_code)]
    pub fn sleep(&self) -> XCapResult<()> {
        let mut parking = self.parking.lock()?;
        *parking = true;

        Ok(())
    }
    #[allow(dead_code)]
    pub fn wait(&self) -> XCapResult<()> {
        let mut parking = self.parking.lock()?;
        while *parking {
            parking = self.condvar.wait(parking)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct VideoRecorder {
    impl_video_recorder: ImplVideoRecorder,
}

impl VideoRecorder {
    pub(crate) fn new(impl_video_recorder: ImplVideoRecorder) -> VideoRecorder {
        VideoRecorder {
            impl_video_recorder,
        }
    }
}

impl VideoRecorder {
    pub fn start(&self) -> XCapResult<()> {
        self.impl_video_recorder.start()
    }
    pub fn stop(&self) -> XCapResult<()> {
        self.impl_video_recorder.stop()
    }
}
