use bitflags::bitflags;
use core::str::FromStr;
use std::ffi::{CStr, CString};
use std::io;
use std::sync::Arc;

use dotlottie_player_core::{Config, Fit, Layout, Marker, Mode};

#[cbindgen::define]
pub const MAX_STR_LENGTH: usize = 128;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(C)]
    pub struct ListenerType: u16 {
        const POINTER_UP    = 1 << 0;
        const POINTER_DOWN  = 1 << 1;
        const POINTER_ENTER = 1 << 2;
        const POINTER_EXIT  = 1 << 3;
        const POINTER_MOVE  = 1 << 4;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ListenerTypeParseError;

impl FromStr for ListenerType {
    type Err = ListenerTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PointerUp" => Ok(ListenerType::POINTER_UP),
            "PointerDown" => Ok(ListenerType::POINTER_DOWN),
            "PointerEnter" => Ok(ListenerType::POINTER_ENTER),
            "PointerExit" => Ok(ListenerType::POINTER_EXIT),
            "PointerMove" => Ok(ListenerType::POINTER_MOVE),
            _ => Err(ListenerTypeParseError()),
        }
    }
}

impl ListenerType {
    pub fn new(listener_types: &Vec<String>) -> Result<ListenerType, ListenerTypeParseError> {
        let mut result: ListenerType;
        for &listener_type in listener_types {
            result |= ListenerType::from_str(&listener_type)?;
        }
        Ok(result)
    }
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieMarker {
    pub name: [u8; MAX_STR_LENGTH],
    pub duration: f32,
    pub time: f32,
}

impl DotLottieMarker {
    pub unsafe fn new(marker: Marker) -> Result<DotLottieMarker> {
        let name: [u8; MAX_STR_LENGTH];
        to_native_string(&marker.name, name, MAX_STR_LENGTH)?;
        Ok(DotLottieMarker {
            name,
            duration: marker.duration,
            time: marker.time,
        })
    }

    pub unsafe fn copy(&self, buffer: *mut DotLottieMarker) {
        std::ptr::copy_nonoverlapping(self as *const DotLottieMarker, buffer, std::mem::size_of<DotLottieMarker>());
    }

    pub unsafe fn copy_all(values: &Vec<DotLottieMarker>, buffer: *mut DotLottieMarker) {
        for value in values{
            copy(value, buffer);
            buffer += 1;
        }
    }
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieLayout {
    pub fit: Fit,
    pub align_x: f32,
    pub align_y: f32,
}

impl DotLottieLayout {
    pub fn new(layout: Layout) {
        let (align_x, align_y) = match config.align {
            [align_x, align_y] => (align_x, align_y),
            _ => (-1.0, -1.0),
        };
        DotLottieLayout {
            fit: layout.fit,
            align_x,
            align_y,
        }
    }

    pub fn to_layout(self) {
        Layout {
            fit: self.fit,
            align: if self.align_x >= 0 && self.align_y >= 0 {
                vec![self.align_x, self.align_y]
            } else {
                vec![]
            },
        }
    }
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieConfig {
    pub mode: Mode,
    pub loop_animation: bool,
    pub speed: f32,
    pub use_frame_interpolation: bool,
    pub autoplay: bool,
    pub segment_start: f32,
    pub segment_end: f32,
    pub background_color: u32,
    pub layout: DotLottieLayout,
    pub marker: [u8; MAX_STR_LENGTH],
}

impl DotLottieConfig {
    pub unsafe fn new(config: Config) -> Result<DotLottieConfig> {
        let (segment_start, segment_end) = match config.segment {
            [start, end] => (start, end),
            _ => (-1.0, -1.0),
        };
        let marker: [u8; MAX_STR_LENGTH];
        to_native_string(&config.marker, marker, MAX_STR_LENGTH)?;
        Ok(DotLottieConfig {
            mode: config.mode,
            loop_animation: config.loop_animation,
            speed: config.speed,
            use_frame_interpolation: config.use_frame_interpolation,
            autoplay: config.autoplay,
            segment_start,
            segment_end,
            background_color: config.background_color,
            layout: DotLottieLayout::to_layout(config.layout),
            marker,
        })
    }

    pub unsafe fn to_config(self) -> Result<Config> {
        Ok(Config {
            mode: self.mode,
            loop_animation: self.loop_animation,
            speed: self.speed,
            use_frame_interpolation: self.use_frame_interpolation,
            autoplay: self.autoplay,
            segment: if self.segment_start >= 0 && self.segment_end >= 0 {
                vec![self.segment_start, self.segment_end]
            } else {
                vec![]
            },
            background_color: self.background_color,
            layout: to_layout(self.layout),
            marker: to_string(self.marker.as_ptr())?,
        })
    }

    pub unsafe fn copy(&self, buffer: *mut DotLottieConfig) {
        std::ptr::copy_nonoverlapping(self as *const DotLottieConfig, buffer, std::mem::size_of<DotLottieConfig>());
    }
}

pub unsafe fn to_string(value: *mut i8) -> Result<String> {
    if value.is_null() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "null pointer"));
    }
    match CStr::from_ptr(value).to_str() {
        Ok(s) => Ok(s.to_owned()),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid utf8 sequence",
        )),
    }
}

pub unsafe fn to_native_string(value: &str, buffer: *mut u8, size: usize) -> Result<()> {
    let bytes = CString::new(value)
        .map_err(|e| Err(io::Error::new(io::ErrorKind::InvalidInput, "null pointer")))?
        .as_bytes_with_nul();
    if bytes.len() <= size {
        Ok(std::ptr::copy_nonoverlapping(bytes, buffer, bytes.len()))
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "value too large",
        ))
    }
}

// Function pointer types
pub type OnOp = unsafe extern "C" fn();

pub type OnFrameOp = unsafe extern "C" fn(f32);
pub type OnRenderOp = unsafe extern "C" fn(f32);
pub type OnLoopOp = unsafe extern "C" fn(u32);

// Observers
#[repr(C)]
pub struct Observer {
    pub on_load_op: OnOp,
    pub on_load_error_op: OnOp,
    pub on_play_op: OnOp,
    pub on_pause_op: OnOp,
    pub on_stop_op: OnOp,
    pub on_frame_op: OnFrameOp,
    pub on_render_op: OnRenderOp,
    pub on_loop_op: OnLoopOp,
    pub on_complete_op: OnOp,
}

impl dotlottie_player_core::Observer for Observer {
    fn on_load(&self) {
        unsafe { (self.on_load_op)() }
    }
    fn on_load_error(&self) {
        unsafe { (self.on_load_error_op)() }
    }
    fn on_play(&self) {
        unsafe { (self.on_play_op)() }
    }
    fn on_pause(&self) {
        unsafe { (self.on_pause_op)() }
    }
    fn on_stop(&self) {
        unsafe { (self.on_stop_op)() }
    }
    fn on_frame(&self, frame_no: f32) {
        unsafe { (self.on_frame_op)(frame_no) }
    }
    fn on_render(&self, frame_no: f32) {
        unsafe { (self.on_render_op)(frame_no) }
    }
    fn on_loop(&self, loop_count: u32) {
        unsafe { (self.on_loop_op)(loop_count) }
    }
    fn on_complete(&self) {
        unsafe { (self.on_complete_op)() }
    }
}

impl Observer {
    pub unsafe fn to_observer(&self) -> Arc<dyn dotlottie_player_core::Observer> {
        Arc::from(Box::from_raw(
            self as *mut dyn dotlottie_player_core::Observer,
        ))
    }
}

pub type OnTransitionOp = unsafe extern "C" fn(*mut i8, *mut i8);
pub type OnStateEnteredOp = unsafe extern "C" fn(*mut i8);
pub type OnStateExitOp = unsafe extern "C" fn(*mut i8);

#[repr(C)]
pub struct StateMachineObserver {
    pub on_transition_op: OnTransitionOp,
    pub on_state_entered_op: OnStateEnteredOp,
    pub on_state_exit_op: OnStateExitOp,
}

impl dotlottie_player_core::StateMachineObserver for StateMachineObserver {
    fn on_transition(&self, previous_state: String, new_state: String) {
        if let Ok(previous_state) = to_mut_i8(&previous_state) {
            if let Ok(new_state) = to_mut_i8(&new_state) {
                unsafe { (self.on_transition_op)(previous_state, new_state) }
            }
        }
    }
    fn on_state_entered(&self, entering_state: String) {
        if let Ok(entering_state) = to_mut_i8(&entering_state) {
            unsafe { (self.on_state_entered_op)(entering_state) }
        }
    }
    fn on_state_exit(&self, leaving_state: String) {
        if let Ok(leaving_state) = to_mut_i8(&leaving_state) {
            unsafe { (self.on_state_exit_op)(leaving_state) }
        }
    }
}

impl StateMachineObserver {
    pub unsafe fn to_observer(&self) -> Arc<dyn dotlottie_player_core::StateMachineObserver> {
        Arc::from(Box::from_raw(
            self as *mut dyn dotlottie_player_core::StateMachineObserver,
        ))
    }
}
