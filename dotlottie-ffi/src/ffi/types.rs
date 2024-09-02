use bitflags::bitflags;
use dotlottie_player_core::{Observer, StateMachineObserver};
use std::ffi::{CStr, CString};
use std::sync::Arc;
use std::{io, ptr};

use dotlottie_player_core::{Config, Fit, Layout, Marker, Mode};

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
    pub fn new(&listener_types: Vec<String>) -> Result<ListenerType, ListenerTypeParseError> {
        let mut result: ListenerType = 0;
        for &listener_type in listener_types {
            result |= ListenerType::from_str(listener_type)?;
        }
        Ok(result)
    }
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieFloatArray {
    pub ptr: *mut f32,
    pub size: usize,
}

impl DotLottieFloatArray {
    pub fn new(floats: Vec<f32>) -> Self {
        let slice = floats.into_boxed_slice();
        std::mem::forget(slice);

        DotLottieFloatArray {
            ptr: slice,
            size: slice.len(),
        }
    }
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieMarker {
    pub name: *mut i8,
    pub duration: f32,
    pub time: f32,
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieMarkerArray {
    pub ptr: *mut DotLottieMarker,
    pub size: usize,
}

impl DotLottieMarkerArray {
    pub fn new(markers: Vec<Marker>) -> DotLottieMarkerArray {
        let slice = markers.into_boxed_slice();
        std::mem::forget(slice);

        DotLottieMarkerArray {
            ptr: slice,
            size: slice.len(),
        }
    }
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieLayout {
    pub fit: Fit,
    pub align: DotLottieFloatArray,
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieConfig {
    pub mode: Mode,
    pub loop_animation: bool,
    pub speed: f32,
    pub use_frame_interpolation: bool,
    pub autoplay: bool,
    pub segment: DotLottieFloatArray,
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

pub unsafe fn to_string(value: *mut i8) -> Result<String, io::Error> {
    if value.is_null() {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "null pointer"))
    } else {
        CStr::from_ptr(value).to_str().map(String::to_owned)
    }
}

pub fn to_mut_i8(value: &str) -> Result<*mut i8, io::Error> {
    CString::new(value)
        .map(CString::into_raw)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
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
        if let Some(previous_state) = to_mut_i8(&previous_state) &&
           let Some(new_state) = to_mut_i8(&new_state) {
            unsafe { (self.on_transition_op)(previous_state, new_state) }
        }
    }
    fn on_state_entered(&self, entering_state: String) {
        if let Some(entering_state) = to_mut_i8(&entering_state) {
            unsafe { (self.on_state_entered_op)(entering_state) }
        }
    }
    fn on_state_exit(&self, leaving_state: String) {
        if let Some(leaving_state) = to_mut_i8(&leaving_state) {
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
