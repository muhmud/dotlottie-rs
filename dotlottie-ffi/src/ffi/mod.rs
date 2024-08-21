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
