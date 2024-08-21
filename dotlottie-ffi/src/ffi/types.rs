use std::ffi::CString;

use dotlottie_player_core::{Config, Fit, Layout, Mode};

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieFloatData {
    pub ptr: *mut f32,
    pub size: usize,
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieLayout {
    pub fit: Fit,
    pub align: DotLottieFloatData,
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieConfig {
    pub mode: Mode,
    pub loop_animation: bool,
    pub speed: f32,
    pub use_frame_interpolation: bool,
    pub autoplay: bool,
    pub segment: DotLottieFloatData,
    pub background_color: u32,
    pub layout: DotLottieLayout,
    pub marker: *mut i8,
}

impl DotLottieConfig {
    pub unsafe fn to_config(&self) -> Config {
        Config {
            mode: self.mode,
            loop_animation: self.loop_animation,
            speed: self.speed,
            use_frame_interpolation: self.use_frame_interpolation,
            autoplay: self.autoplay,
            segment: Vec::from_raw_parts(self.segment.ptr, self.segment.size, self.segment.size),
            background_color: self.background_color,
            layout: Layout {
                fit: self.layout.fit.clone(),
                align: Vec::from_raw_parts(self.segment.ptr, self.segment.size, self.segment.size),
            },
            marker: to_string(self.marker),
        }
    }
}

pub unsafe fn to_string(value: *mut i8) -> String {
    if value.is_null() {
        String::new()
    } else {
        CString::from_raw(value).to_str().unwrap_or("").to_owned()
    }
}
