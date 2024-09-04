use std::ffi::c_void;

use dotlottie_player_core::{Config, DotLottiePlayer};
use types::{to_native_string, to_string, DotLottieConfig, ListenerType, MAX_STR_LENGTH};

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

fn to_exit_status(result: bool) -> i32 {
    if result {
        EXIT_SUCCESS
    } else {
        EXIT_ERROR
    }
}

#[no_mangle]
pub unsafe extern "C" fn new_dotlottie_player(ptr: *const DotLottieConfig) -> *mut DotLottiePlayer {
    if let Some(dotlottie_config) = ptr.as_ref() {
        if let Ok(config) = dotlottie_config.to_config() {
            let dotlottie_player = Box::new(DotLottiePlayer::new(config));
            return Box::into_raw(dotlottie_player);
        }
    }
    std::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_player_load_animation_data(
    ptr: *mut DotLottiePlayer,
    animation_data: *mut i8,
    width: u32,
    height: u32,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| match to_string(animation_data) {
        Some(&animation_data) => {
            to_exit_status(dotlottie_player.load_animation_data(animation_data, width, height))
        }
        _ => EXIT_ERROR,
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_player_load_animation_path(
    ptr: *mut DotLottiePlayer,
    animation_path: *mut i8,
    width: u32,
    height: u32,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| match to_string(animation_path) {
        Some(&animation_path) => {
            to_exit_status(dotlottie_player.load_animation_path(animation_path, width, height))
        }
        _ => EXIT_ERROR,
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_player_load_dotlottie_data(
    ptr: *mut DotLottiePlayer,
    file_data: *const u8,
    file_size: usize,
    width: u32,
    height: u32,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        let file_slice = slice::from_raw_parts(file_data, file_size);
        to_exit_status(dotlottie_player.load_dotlottie_data(file_slice, width, height))
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_player_load_animation(
    ptr: *mut DotLottiePlayer,
    animation_id: *mut i8,
    width: u32,
    height: u32,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| match to_string(animation_id) {
        Some(&animation_id) => {
            to_exit_status(dotlottie_player.load_animation(animation_id, width, height))
        }
        _ => EXIT_ERROR,
    })
}

/////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn dotlottie_manifest_string(
    ptr: *mut DotLottiePlayer,
    result: *mut i8,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        if let Ok(manifest_string) = types::to_mut_i8(&dotlottie_player.manifest_string()) {
            result = manifest_string;
            EXIT_SUCCESS
        } else {
            EXIT_ERROR
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_buffer_ptr(
    ptr: *mut DotLottiePlayer,
    result: *mut *const u32,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.buffer();
        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_buffer_len(ptr: *mut DotLottiePlayer, result: *mut u64) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.buffer_len();
        EXIT_SUCCESS
    })
}

/////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn dotlottie_config(
    ptr: *mut DotLottiePlayer,
    result: *mut DotLottieConfig,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        if let Ok(config) = DotLottieConfig::new(dotlottie_player.config()) {
            std::ptr::copy_nonoverlapping(
                config as *const u8,
                result,
                std::mem::size_of::<DotLottieConfig>(),
            );
            EXIT_SUCCESS
        } else {
            EXIT_ERROR
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_total_frames(
    ptr: *mut DotLottiePlayer,
    result: *mut f32,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.total_frames();
        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_duration(ptr: *mut DotLottiePlayer, result: *mut f32) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.duration();
        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_current_frame(
    ptr: *mut DotLottiePlayer,
    result: *mut f32,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.current_frame();
        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_loop_count(ptr: *mut DotLottiePlayer) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.loop_count())
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_loaded(ptr: *mut DotLottiePlayer) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.is_loaded())
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_playing(ptr: *mut DotLottiePlayer) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.is_playing())
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_paused(ptr: *mut DotLottiePlayer) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.is_paused())
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_stopped(ptr: *mut DotLottiePlayer) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.is_stopped())
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_play(ptr: *mut DotLottiePlayer) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.play())
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_pause(ptr: *mut DotLottiePlayer) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.pause())
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_stop(ptr: *mut DotLottiePlayer) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.stop())
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_request_frame(
    ptr: *mut DotLottiePlayer,
    result: *mut f32,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.request_frame();
        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_set_frame(ptr: *mut DotLottiePlayer, no: f32) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.set_frame(no))
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_seek(ptr: *mut DotLottiePlayer, no: f32) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.seek(no))
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_render(ptr: *mut DotLottiePlayer) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.render())
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_resize(
    ptr: *mut DotLottiePlayer,
    width: u32,
    height: u32,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.resize(width, height))
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_clear(ptr: *mut DotLottiePlayer) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        dotlottie_player.clear();
        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_complete(
    ptr: *mut DotLottiePlayer,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.is_complete())
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_load_theme(ptr: *mut DotLottiePlayer, theme_id: *mut i8) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| match to_string(theme_id) {
        Some(theme_id) => to_exit_status(dotlottie_player.load_theme(&theme_id)),
        _ => EXIT_ERROR,
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_load_theme_data(
    ptr: *mut DotLottiePlayer,
    theme_data: *mut i8,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| match to_string(theme_data) {
        Some(theme_data) => to_exit_status(dotlottie_player.load_theme_data(&theme_data)),
        _ => EXIT_ERROR,
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_markers(
    ptr: *mut DotLottiePlayer,
    result: *mut DotLottieMarker,
    size: *mut usize,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        let markers = dotlottie_player.markers();
        if size.is_null() {
            // Size must always be specified
            EXIT_ERROR
        } else if result.is_null() {
            // Return the number of items that will be returned
            *size = markers.len();
            EXIT_SUCCESS
        } else {
            if *size < markers.len() {
                // The size of the buffer must be big enough to hold the result
                EXIT_ERROR
            } else {
                DotLottieMarker::copy_all(markers, result);
                *size = markers.len();
                EXIT_SUCCESS
            }
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_active_animation_id(
    ptr: *mut DotLottiePlayer,
    result: *mut u8,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        if let Ok(_) = to_native_string(
            &dotlottie_player.active_animation_id(),
            result,
            MAX_STR_LENGTH,
        ) {
            EXIT_SUCCESS
        } else {
            EXIT_ERROR
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_active_theme_id(
    ptr: *mut DotLottiePlayer,
    result: *mut u8,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        if let Ok(_) = to_native_string(&dotlottie_player.active_theme_id(), result, MAX_STR_LENGTH)
        {
            EXIT_SUCCESS
        } else {
            EXIT_ERROR
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_set_viewport(
    ptr: *mut DotLottiePlayer,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.set_viewport(x, y, w, h))
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_segment_duration(
    ptr: *mut DotLottiePlayer,
    result: *mut f32,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.segment_duration();
        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_animation_size(
    ptr: *mut DotLottiePlayer,
    width: *mut f32,
    height: *mut f32,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        match dotlottie_player.animation_size().as_slice() {
            [picture_width, picture_height] => {
                *width = picture_width;
                *height = picture_height;
                EXIT_SUCCESS
            }
            _ => EXIT_ERROR,
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_load_state_machine(
    ptr: *mut DotLottiePlayer,
    state_machine_id: *mut i8,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| match to_string(state_machine_id) {
        Some(state_machine_id) => {
            to_exit_status(dotlottie_player.load_state_machine(&state_machine_id))
        }
        _ => EXIT_ERROR,
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_start_state_machine(ptr: *mut DotLottiePlayer) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.start_state_machine())
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_stop_state_machine(ptr: *mut DotLottiePlayer) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.stop_state_machine())
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_set_state_machine_numeric_context(
    ptr: *mut DotLottiePlayer,
    key: *mut i8,
    value: f32,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| match to_string(key) {
        Some(&key) => {
            to_exit_status(dotlottie_player.set_state_machine_numeric_context(key, value))
        }
        _ => EXIT_ERROR,
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_set_state_machine_string_context(
    ptr: *mut DotLottiePlayer,
    key: *mut i8,
    value: *mut i8,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        match (to_string(key), to_string(value)) {
            (Some(&key), Some(&value)) => {
                to_exit_status(dotlottie_player.set_state_machine_string_context(key, value))
            }
            _ => EXIT_ERROR,
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_set_state_machine_boolean_context(
    ptr: *mut DotLottiePlayer,
    key: *mut i8,
    value: bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| match to_string(key) {
        Some(&key) => {
            to_exit_status(dotlottie_player.set_state_machine_boolean_context(key, value))
        }
        _ => EXIT_ERROR,
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_state_machine_framework_setup(
    ptr: *mut DotLottiePlayer,
    result: *mut ListenerType,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(ListenerType::new(
            &dotlottie_player.state_machine_framework_setup(),
        ))
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_load_state_machine_data(
    ptr: *mut DotLottiePlayer,
    state_machine_definition: *mut i8,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        match to_string(state_machine_definition) {
            Some(&state_machine_definition) => {
                to_exit_status(dotlottie_player.load_state_machine_data(state_machine_definition))
            }
            _ => EXIT_ERROR,
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_player_subscribe(
    ptr: *mut DotLottiePlayer,
    observer: *mut types::Observer,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        dotlottie_player.subscribe(observer.to_observer());
        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_player_unsubscribe(
    ptr: *mut DotLottiePlayer,
    observer: *mut types::Observer,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        dotlottie_player.unsubscribe(observer.to_observer());
        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_player_state_machine_subscribe(
    ptr: *mut DotLottiePlayer,
    observer: *mut types::StateMachineObserver,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.state_machine_subscribe(observer.to_observer()))
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_player_state_machine_unsubscribe(
    ptr: *mut DotLottiePlayer,
    observer: *mut types::StateMachineObserver,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        to_exit_status(dotlottie_player.state_machine_unsubscribe(observer.to_observer()))
    })
}
