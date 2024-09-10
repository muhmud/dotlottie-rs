use bitflags::bitflags;
use core::str::FromStr;
use std::ffi::{CStr, CString};
use std::io;
use std::sync::Arc;

use dotlottie_fms::{Manifest, ManifestAnimation, ManifestTheme};
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
pub struct DotLottieFloatArray {
    pub ptr: *mut f32,
    pub size: usize,
}

impl DotLottieFloatArray {
    pub fn new(floats: Vec<f32>) -> Self {
        let mut slice = floats.into_boxed_slice();
        std::mem::forget(slice);

        DotLottieFloatArray {
            ptr: slice.as_mut_ptr(),
            size: slice.len(),
        }
    }
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieMuti8Array {
    pub ptr: *mut i8,
    pub size: usize,
}

impl DotLottieMuti8Array {
    pub fn new(muti8s: Vec<str>) -> Self {
        let mut slice = muti8s.into_boxed_slice();
        std::mem::forget(slice);
        let result = to_mut_i8(slice);

        DotLottieMuti8Array {
            ptr: to_mut_i8(Ok(Result)),
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

pub struct DotLottieManifestAnimation {
    pub autoplay: bool,
    pub default_theme: *mut i8,
    pub direction: i8,
    pub hover: bool,
    pub id: *mut i8,
    pub intermission: u32,
    pub r#loop: bool,
    pub loop_count: u32,
    pub play_mode: *mut i8,
    pub speed: f32,
    pub theme_color: *mut i8,
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieManifestAnimationArray {
    pub ptr: *mut DotLottieManifestAnimation,
    pub size: usize,
}

impl DotLottieManifestAnimationArray {
    pub unsafe fn new(manifests: Vec<ManifestAnimation>) -> Self {
        let mut slice = manifests.into_boxed_slice();
        std::mem::forget(slice);
        let swv = (*slice.as_mut_ptr());

        DotLottieManifestAnimationArray {
            ptr: DotLottieManifestAnimation {
                autoplay: swv.autoplay,
                default_theme: Ok(to_mut_i8(swv.defaultTheme)),
                direction: swv.direction,
                hover: swv.hover,
                id: Ok(to_mut_i8(&swv.id)),
                intermission: swv.intermission,
                r#loop: swv.r#loop,
                loop_count: swv.loop_count,
                play_mode: Ok(to_mut_i8(swv.playMode)),
                speed: swv.speed,
                theme_color: Ok(to_mut_i8(swv.themeColor)),
            },
            size: slice.len(),
        }
    }
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieManifestTheme {
    pub id: *mut i8,
    pub animations: DotLottieMuti8Array,
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieManifestThemeArray {
    pub ptr: *mut DotLottieManifestTheme,
    pub size: usize,
}

impl DotLottieManifestThemeArray {
    pub unsafe fn new(manifest_themes: Vec<ManifestTheme>) -> Self {
        let mut slice = manifest_themes.into_boxed_slice();
        std::mem::forget(slice);
        let swv = (*slice.as_mut_ptr());

        DotLottieManifestThemeArray {
            ptr: DotLottieManifestTheme {
                id: swv.id,
                animations: DotLottieMuti8Array::new(swv.animations)
            },
            size: slice.len(),
        }
    }
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieManifest {
    pub active_animation_id: *mut i8,
    pub animations: DotLottieManifestAnimationArray,
    pub author: *mut i8,
    pub description: *mut i8,
    pub generator: *mut i8,
    pub keywords: *mut i8,
    pub revision: u32,
    pub themes: DotLottieManifestThemeArray,
    pub states: DotLottieMuti8Array,
    pub version: *mut i8,
}

impl DotLottieManifest {
    pub unsafe fn new(manifest: Manifest) -> DotLottieManifest {
        let dl_manifest: DotLottieManifest = DotLottieManifest {
            active_animation_id: Ok(manifest.active_animation_id),
            animations: DotLottieManifestAnimationArray::new(manifest.animations),
            author: to_mut_i8(manifest.author),
            description: to_mut_i8(manifest.description),
            generator: to_mut_i8(manifest.generator),
            keywords: to_mut_i8(manifest.keywords),
            revision: manifest.revision,
            themes: DotLottieManifestThemeArray::new(manifest.themes),
            states: DotLottieMuti8Array::new(manifest.states),
            version: to_mut_i8(manifest.version)
        };
    }
}

#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct DotLottieMarkerArray {
    pub ptr: *mut DotLottieMarker,
    pub size: usize,
}

impl DotLottieMarkerArray {
    pub unsafe fn new(markers: Vec<Marker>) -> DotLottieMarkerArray {
        let mut slice = markers.into_boxed_slice();
        let dotlottie_marker: *mut DotLottieMarker;
        let name = (*slice.as_mut_ptr()).name;
        if let Ok(name) = to_mut_i8(&name) {
            *dotlottie_marker = DotLottieMarker {
                name,
                duration: (*slice.as_mut_ptr()).duration,
                time: (*slice.as_mut_ptr()).time,
            };
        }
        std::mem::forget(slice);

        DotLottieMarkerArray {
            ptr: dotlottie_marker,
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
        let mut config = Config {
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
            marker: String::new(),
        };

        let marker = self.marker;
        if let Ok(marker) = to_string(marker) {
            config.marker = marker
        }
        config
    }
}
pub unsafe fn to_string(value: *mut i8) -> Result<String, io::Error> {
    if value.is_null() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "null pointer"));
    }

    // Safely convert the C string to a Rust String
    match CStr::from_ptr(value).to_str() {
        Ok(s) => Ok(s.to_owned()),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid UTF-8 sequence",
        )),
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
