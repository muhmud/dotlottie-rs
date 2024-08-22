use dotlottie_fms::Manifest;
use dotlottie_player_core::{Config, DotLottiePlayer, Event, Marker};
use types::DotLottieConfig;
use uniffi;

pub mod types;

const EXIT_SUCCESS: i32 = 0;
const EXIT_ERROR: i32 = 1;

unsafe fn exec_dotlottie_player_op<Op>(ptr: *mut DotLottiePlayer, op: Op) -> i32
where
    Op: Fn(&DotLottiePlayer) -> i32,
{
    match ptr.as_ref() {
        Some(dotlottie_player) => op(&dotlottie_player),
        _ => EXIT_ERROR,
    }
}

#[no_mangle]
pub unsafe extern "C" fn new_dotlottie_player(ptr: *const DotLottieConfig) -> *mut DotLottiePlayer {
    if let Some(dotlottie_config) = ptr.as_ref() {
        let config = dotlottie_config.to_config();
        let dotlottie_player = Box::new(DotLottiePlayer::new(config));
        Box::into_raw(dotlottie_player)
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_player_load_animation_data(
    ptr: *mut DotLottiePlayer,
    animation_data_str: *mut i8,
    width: u32,
    height: u32,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        let animation_data = types::to_string(animation_data_str);
        *result = dotlottie_player.load_animation_data(&animation_data, width, height);

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_player_load_animation_path(
    ptr: *mut DotLottiePlayer,
    animation_path_str: *mut i8,
    width: u32,
    height: u32,
) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        let animation_path = types::to_string(animation_path_str);
        dotlottie_player.load_animation_path(&animation_path, width, height)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_player_load_dotlottie_data(
    ptr: *mut DotLottiePlayer,
    file_data: uniffi::deps::bytes,
    width: u32,
    height: u32,
) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.load_dotlottie_data(&file_data, width, height)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_player_load_animation(
    ptr: *mut DotLottiePlayer,
    animation_id_str: *mut i8,
    width: u32,
    height: u32,
) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        let animation_id = types::to_string(animation_id_str);
        dotlottie_player.load_animation(&animation_id, width, height)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_manifest(ptr: *mut DotLottiePlayer) -> Manifest {
    // When I put an ? here it says expected item what does that mean?
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.manifest()
    } else {
        Manifest
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_manifest_string(ptr: *mut DotLottiePlayer) -> String {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.manifest_string()
    } else {
        String::from("")
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_buffer_ptr(ptr: *mut DotLottiePlayer) -> u64 {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.buffer_ptr()
    } else {
        u64::from(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_buffer_len(ptr: *mut DotLottiePlayer) -> u64 {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.buffer_len()
    } else {
        u64::from(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_config(ptr: *mut DotLottiePlayer) -> Config {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.config()
    } else {
        Config
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_total_frames(ptr: *mut DotLottiePlayer) -> f32 {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.total_frames()
    } else {
        f32::from(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_duration(ptr: *mut DotLottiePlayer) -> f32 {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.duration()
    } else {
        f32::from(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_current_frame(ptr: *mut DotLottiePlayer) -> f32 {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.current_frame()
    } else {
        f32::from(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_loop_count(ptr: *mut DotLottiePlayer) -> u32 {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.loop_count()
    } else {
        u32::from(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_loaded(ptr: *mut DotLottiePlayer) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.is_loaded()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_playing(ptr: *mut DotLottiePlayer) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.is_playing()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_playing(ptr: *mut DotLottiePlayer) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.is_playing()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_paused(ptr: *mut DotLottiePlayer) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.is_paused()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_stopped(ptr: *mut DotLottiePlayer) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.is_stopped()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_play(ptr: *mut DotLottiePlayer) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.play()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_pause(ptr: *mut DotLottiePlayer) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.pause()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_stop(ptr: *mut DotLottiePlayer) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.stop()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_request_frame(ptr: *mut DotLottiePlayer) -> f32 {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.request_frame()
    } else {
        f32::from(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_set_frame(ptr: *mut DotLottiePlayer, no: f32) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.set_frame(no)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_seek(ptr: *mut DotLottiePlayer, no: f32) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.seek(no)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_render(ptr: *mut DotLottiePlayer) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.render()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_resize(
    ptr: *mut DotLottiePlayer,
    width: u32,
    height: u32,
) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.resize(width, height)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_clear(ptr: *mut DotLottiePlayer) {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.clear()
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_complete(ptr: *mut DotLottiePlayer) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.is_complete()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_complete(ptr: *mut DotLottiePlayer) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.is_complete()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_load_theme(
    ptr: *mut DotLottiePlayer,
    theme_id_str: *mut i8,
) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        let theme_id = types::to_string(theme_id_str);
        dotlottie_player.load_theme(&theme_id)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_load_theme(
    ptr: *mut DotLottiePlayer,
    theme_data_str: *mut i8,
) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        let theme_data = types::to_string(theme_data_str);
        dotlottie_player.load_theme_data(&theme_data)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_markers(ptr: *mut DotLottiePlayer) -> vec<Marker> {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.markers()
    } else {
        Some(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_active_animation_id(ptr: *mut DotLottiePlayer) -> String {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.active_animation_id()
    } else {
        Some("")
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_active_theme_id(ptr: *mut DotLottiePlayer) -> String {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.active_theme_id()
    } else {
        Some("")
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_set_viewport(
    ptr: *mut DotLottiePlayer,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.set_viewport(x, y, w, h)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_segment_duration(ptr: *mut DotLottiePlayer) -> f32 {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.segment_duration()
    } else {
        Some(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_animation_size(ptr: *mut DotLottiePlayer) -> vec<f32> {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.animation_size()
    } else {
        Some(0)
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_load_state_machine(
    ptr: *mut DotLottiePlayer,
    str: *mut i8,
) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        let str2 = types::to_string(str);
        dotlottie_player.load_state_machine(&str2)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_start_state_machine(ptr: *mut DotLottiePlayer) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.start_state_machine()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_stop_state_machine(ptr: *mut DotLottiePlayer) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.stop_state_machine()
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_set_state_machine_numeric_context(
    ptr: *mut DotLottiePlayer,
    key_str: *mut i8,
    value: f32,
) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        let key = types::to_string(key_str);
        dotlottie_player.set_state_machine_numeric_context(&key, value)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_set_state_machine_string_context(
    ptr: *mut DotLottiePlayer,
    key_str: *mut i8,
    value_str: *mut i8,
) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        let key = types::to_string(key_str);
        let value = types::to_string(value_str);
        dotlottie_player.set_state_machine_numeric_context(&key, &value)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_set_state_machine_boolean_context(
    ptr: *mut DotLottiePlayer,
    key_str: *mut i8,
    value: bool,
) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        let key = types::to_string(key_str);
        dotlottie_player.set_state_machine_boolean_context(&key, value)
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_state_machine_framework_setup(
    ptr: *mut DotLottiePlayer,
) -> vec<String> {
    if let Some(dotlottie_player) = ptr.as_ref() {
        dotlottie_player.state_machine_framework_setup()
    } else {
        Some("")
    }
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_load_state_machine_data(
    ptr: *mut DotLottiePlayer,
    state_machine_str: *mut i8,
) -> vec<String> {
    if let Some(dotlottie_player) = ptr.as_ref() {
        let state_machine = types::to_string(state_machine_str);
        dotlottie_player.load_state_machine_data(&state_machine)
    } else {
        Some("")
    }
}
