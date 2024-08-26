use std::ffi::CString;
use std::ptr;

use dotlottie_player_core::{Config, Fit, Layout, Mode, Marker};

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
        ptr::null_mut()  // Return a null pointer if the string is empty
    } else {
        let c_string = CString::new(value).expect("CString::new failed");
        c_string.into_raw()  // Convert the CString into a raw pointer
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
