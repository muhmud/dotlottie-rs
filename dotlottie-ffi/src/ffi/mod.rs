use dotlottie_player_core::DotLottiePlayer;
use types::DotLottieConfig;

pub mod types;

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
) -> bool {
    if let Some(dotlottie_player) = ptr.as_ref() {
        let animation_data = types::to_string(animation_data_str);
        dotlottie_player.load_animation_data(&animation_data, width, height)
    } else {
        false
    }
}
