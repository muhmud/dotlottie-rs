use dotlottie_player_core::{Config, DotLottiePlayer, Marker};
use types::DotLottieConfig;

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
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        let animation_path = types::to_string(animation_path_str);
        *result = dotlottie_player.load_animation_path(&animation_path, width, height);

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_player_load_dotlottie_data(
    ptr: *mut DotLottiePlayer,
    file_data: &[u8],
    width: u32,
    height: u32,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.load_dotlottie_data(&file_data, width, height);

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_player_load_animation(
    ptr: *mut DotLottiePlayer,
    animation_id_str: *mut i8,
    width: u32,
    height: u32,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        let animation_id = types::to_string(animation_id_str);
        *result = dotlottie_player.load_animation(&animation_id, width, height);

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_manifest_string(
    ptr: *mut DotLottiePlayer,
    result: *mut i8,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = *types::to_mut_i8(dotlottie_player.manifest_string());

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_buffer_ptr(ptr: *mut DotLottiePlayer, result: *mut u64) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.buffer_ptr();

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

#[no_mangle]
pub unsafe extern "C" fn dotlottie_config(ptr: *mut DotLottiePlayer, result: *mut Config) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.config();

        EXIT_SUCCESS
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
pub unsafe extern "C" fn dotlottie_loop_count(ptr: *mut DotLottiePlayer, result: *mut u32) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.loop_count();

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_loaded(ptr: *mut DotLottiePlayer, result: *mut bool) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.is_loaded();

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_playing(ptr: *mut DotLottiePlayer, result: *mut bool) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.is_playing();

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_paused(ptr: *mut DotLottiePlayer, result: *mut bool) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.is_paused();

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_is_stopped(ptr: *mut DotLottiePlayer, result: *mut bool) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.is_stopped();

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_play(ptr: *mut DotLottiePlayer, result: *mut bool) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.play();

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_pause(ptr: *mut DotLottiePlayer, result: *mut bool) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.pause();

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_stop(ptr: *mut DotLottiePlayer, result: *mut bool) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.stop();

        EXIT_SUCCESS
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
pub unsafe extern "C" fn dotlottie_set_frame(
    ptr: *mut DotLottiePlayer,
    no: f32,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.set_frame(no);

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_seek(
    ptr: *mut DotLottiePlayer,
    no: f32,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.seek(no);

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_render(ptr: *mut DotLottiePlayer, result: *mut bool) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.render();

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_resize(
    ptr: *mut DotLottiePlayer,
    width: u32,
    height: u32,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.resize(width, height);

        EXIT_SUCCESS
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
        *result = dotlottie_player.is_complete();

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_load_theme(
    ptr: *mut DotLottiePlayer,
    theme_id_str: *mut i8,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        let theme_id = types::to_string(theme_id_str);
        *result = dotlottie_player.load_theme(&theme_id);

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_load_theme_data(
    ptr: *mut DotLottiePlayer,
    theme_data_str: *mut i8,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        let theme_data = types::to_string(theme_data_str);
        *result = dotlottie_player.load_theme_data(&theme_data);

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_markers(ptr: *mut DotLottiePlayer, result: *mut types::DotLottieMarkerData) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
         let mut markers = dotlottie_player.markers();
         *result = types::vec_markers_to_dotlottiemarkerdata(&mut markers);

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_active_animation_id(
    ptr: *mut DotLottiePlayer,
    result: *mut i8,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = *types::to_mut_i8(dotlottie_player.active_animation_id());

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_active_theme_id(
    ptr: *mut DotLottiePlayer,
    result: *mut i8,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = *types::to_mut_i8(dotlottie_player.active_theme_id());

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_set_viewport(
    ptr: *mut DotLottiePlayer,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.set_viewport(x, y, w, h);

        EXIT_SUCCESS
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
    result: *mut types::DotLottieFloatData,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = types::vec_floats_to_dotlottiefloatdata(dotlottie_player.animation_size());

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_load_state_machine(
    ptr: *mut DotLottiePlayer,
    str: *mut i8,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        let str2 = types::to_string(str);
        *result = dotlottie_player.load_state_machine(&str2);

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_start_state_machine(
    ptr: *mut DotLottiePlayer,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.start_state_machine();

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_stop_state_machine(
    ptr: *mut DotLottiePlayer,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        *result = dotlottie_player.stop_state_machine();

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_set_state_machine_numeric_context(
    ptr: *mut DotLottiePlayer,
    key_str: *mut i8,
    value: f32,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        let key = types::to_string(key_str);
        *result = dotlottie_player.set_state_machine_numeric_context(&key, value);

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_set_state_machine_string_context(
    ptr: *mut DotLottiePlayer,
    key_str: *mut i8,
    value_str: *mut i8,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        let key = types::to_string(key_str);
        let value = types::to_string(value_str);
        *result = dotlottie_player.set_state_machine_string_context(&key, &value);

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_set_state_machine_boolean_context(
    ptr: *mut DotLottiePlayer,
    key_str: *mut i8,
    value: bool,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        let key = types::to_string(key_str);
        *result = dotlottie_player.set_state_machine_boolean_context(&key, value);

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_state_machine_framework_setup(
    ptr: *mut DotLottiePlayer,
    result: *mut types::DotLottiei8Data,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
       *result = types::vec_strings_to_dotlottiei8data(&dotlottie_player.state_machine_framework_setup());

        EXIT_SUCCESS
    })
}

#[no_mangle]
pub unsafe extern "C" fn dotlottie_load_state_machine_data(
    ptr: *mut DotLottiePlayer,
    state_machine_str: *mut i8,
    result: *mut bool,
) -> i32 {
    exec_dotlottie_player_op(ptr, |dotlottie_player| {
        let state_machine = types::to_string(state_machine_str);
        *result = dotlottie_player.load_state_machine_data(&state_machine);

        EXIT_SUCCESS
    })
}
