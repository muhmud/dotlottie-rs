use dotlottie_player_core::{Observer, StateMachineObserver};
use std::ffi::CString;
use std::ptr;
use std::sync::Arc;

use dotlottie_player_core::{Config, Fit, Layout, Marker, Mode};

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieFloatData {
    pub ptr: *mut f32,
    pub size: usize,
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottiei8Data {
    pub ptr: *mut i8,
    pub size: usize,
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieMarkerData {
    pub ptr: *mut Marker,
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

pub unsafe fn to_mut_i8(value: String) -> *mut i8 {
    if value.is_empty() {
        ptr::null_mut() // Return a null pointer if the string is empty
    } else {
        let c_string = CString::new(value).expect("CString::new failed");
        c_string.into_raw() // Convert the CString into a raw pointer
    }
}

pub unsafe fn vec_strings_to_dotlottiei8data(strings: &Vec<String>) -> DotLottiei8Data {
    // Concatenate all strings into one String
    let concatenated = strings.join("");

    // Convert the concatenated String into bytes
    let bytes = concatenated.into_bytes();
    let size = bytes.len();

    // Convert bytes into a Vec<i8>
    let mut vec_i8: Vec<i8> = bytes.into_iter().map(|b| b as i8).collect();

    // Get a raw pointer to the Vec<i8> data
    let ptr = vec_i8.as_mut_ptr();

    // Prevent Rust from deallocating the memory by not using std::mem::forget
    std::mem::forget(vec_i8); // Keep the memory alive

    DotLottiei8Data { ptr, size }
}

pub fn vec_floats_to_dotlottiefloatdata(floats: Vec<f32>) -> DotLottieFloatData {
    let size = floats.len(); // Get the size of the data

    // Box the Vec<f32> to ensure memory is managed properly
    let boxed_slice = floats.into_boxed_slice();

    // Get a raw pointer to the boxed slice
    let ptr = boxed_slice.as_ptr() as *mut f32;

    // Move the boxed slice out of scope to prevent deallocation
    std::mem::forget(boxed_slice);

    DotLottieFloatData { ptr, size }
}

pub unsafe fn vec_markers_to_dotlottiemarkerdata(markers: &mut Vec<Marker>) -> DotLottieMarkerData {
    let size = markers.len(); // Get the size of the data

    // Get a raw pointer to the data and the size
    let ptr = markers.as_mut_ptr();

    DotLottieMarkerData { ptr, size }
}

// Function pointer types
pub type OnOp = unsafe extern "C" fn();

pub type OnFrameOp = unsafe extern "C" fn(f32);
pub type OnRenderOp = unsafe extern "C" fn(f32);
pub type OnLoopOp = unsafe extern "C" fn(u32);

// Observers
#[repr(C)]
pub struct CObserver {
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

impl Observer for CObserver {
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

pub type OnTransitionOp = unsafe extern "C" fn(*mut i8, *mut i8);
pub type OnStateEnteredOp = unsafe extern "C" fn(*mut i8);
pub type OnStateExitOp = unsafe extern "C" fn(*mut i8);

#[repr(C)]
pub struct CStateMachineObserver {
    pub on_transition_op: OnTransitionOp,
    pub on_state_entered_op: OnStateEnteredOp,
    pub on_state_exit_op: OnStateExitOp,
}

impl StateMachineObserver for CStateMachineObserver {
    fn on_transition(&self, previous_state: String, new_state: String) {
        unsafe { (self.on_transition_op)(to_mut_i8(previous_state), to_mut_i8(new_state)) }
    }
    fn on_state_entered(&self, entering_state: String) {
        unsafe { (self.on_state_entered_op)(to_mut_i8(entering_state)) }
    }
    fn on_state_exit(&self, leaving_state: String) {
        unsafe { (self.on_state_exit_op)(to_mut_i8(leaving_state)) }
    }
}

pub unsafe fn cobserver_to_box_of_observer(ptr: *mut CObserver) -> Box<dyn Observer> {
    if ptr.is_null() {
        panic!("Received null pointer");
    }

    // Convert the raw pointer to `Box<dyn Observer>`
    Box::from_raw(ptr as *mut dyn Observer)
}

pub fn box_of_observer_to_arc_of_observer(box_observer: Box<dyn Observer>) -> Arc<dyn Observer> {
    Arc::from(box_observer)
}

pub unsafe fn cstate_machine_observer_to_box_of_state_machine_observer(ptr: *mut CStateMachineObserver) -> Box<dyn StateMachineObserver> {
    if ptr.is_null() {
        panic!("Received null pointer");
    }

    // Convert the raw pointer to `Box<dyn Observer>`
    Box::from_raw(ptr as *mut dyn StateMachineObserver)
}

pub fn box_of_state_machine_observer_to_arc_of_state_machine_observer(state_machine_observer: Box<dyn StateMachineObserver>) -> Arc<dyn StateMachineObserver> {
    Arc::from(state_machine_observer)
}
