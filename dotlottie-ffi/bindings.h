#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef enum Fit {
  Contain,
  Fill,
  Cover,
  FitWidth,
  FitHeight,
  None,
} Fit;

typedef enum Mode {
  Forward,
  Reverse,
  Bounce,
  ReverseBounce,
} Mode;

typedef struct DotLottiePlayer DotLottiePlayer;

typedef struct Layout Layout;

typedef struct ListenerType ListenerType;

typedef struct String String;

typedef struct Vec_f32 Vec_f32;

typedef struct DotLottieFloatArray {
  float *ptr;
  size_t size;
} DotLottieFloatArray;

typedef struct Config {
  enum Mode mode;
  bool loop_animation;
  float speed;
  bool use_frame_interpolation;
  bool autoplay;
  struct Vec_f32 segment;
  uint32_t background_color;
  struct Layout layout;
  struct String marker;
} Config;

typedef struct DotLottieManifestAnimation {
  bool autoplay;
  int8_t *default_theme;
  int8_t direction;
  bool hover;
  int8_t *id;
  uint32_t intermission;
  bool loop;
  uint32_t loop_count;
  int8_t *play_mode;
  float speed;
  int8_t *theme_color;
} DotLottieManifestAnimation;

typedef struct DotLottieManifestAnimationArray {
  struct DotLottieManifestAnimation *ptr;
  size_t size;
} DotLottieManifestAnimationArray;

typedef struct DotLottieMuti8Array {
  int8_t *ptr;
  size_t size;
} DotLottieMuti8Array;

typedef struct DotLottieManifestTheme {
  int8_t *id;
  struct DotLottieMuti8Array animations;
} DotLottieManifestTheme;

typedef struct DotLottieManifestThemeArray {
  struct DotLottieManifestTheme *ptr;
  size_t size;
} DotLottieManifestThemeArray;

typedef struct DotLottieManifest {
  int8_t *active_animation_id;
  struct DotLottieManifestAnimationArray animations;
  int8_t *author;
  int8_t *description;
  int8_t *generator;
  int8_t *keywords;
  uint32_t revision;
  struct DotLottieManifestThemeArray themes;
  struct DotLottieMuti8Array states;
  int8_t *version;
} DotLottieManifest;

typedef struct DotLottieMarker {
  int8_t *name;
  float duration;
  float time;
} DotLottieMarker;

typedef struct DotLottieMarkerArray {
  struct DotLottieMarker *ptr;
  size_t size;
} DotLottieMarkerArray;

typedef void (*OnTransitionOp)(int8_t*, int8_t*);

typedef void (*OnStateEnteredOp)(int8_t*);

typedef void (*OnStateExitOp)(int8_t*);

typedef struct StateMachineObserver {
  OnTransitionOp on_transition_op;
  OnStateEnteredOp on_state_entered_op;
  OnStateExitOp on_state_exit_op;
} StateMachineObserver;

typedef void (*OnOp)(void);

typedef void (*OnFrameOp)(float);

typedef void (*OnRenderOp)(float);

typedef void (*OnLoopOp)(uint32_t);

typedef struct Observer {
  OnOp on_load_op;
  OnOp on_load_error_op;
  OnOp on_play_op;
  OnOp on_pause_op;
  OnOp on_stop_op;
  OnFrameOp on_frame_op;
  OnRenderOp on_render_op;
  OnLoopOp on_loop_op;
  OnOp on_complete_op;
} Observer;

typedef struct DotLottieLayout {
  enum Fit fit;
  struct DotLottieFloatArray align;
} DotLottieLayout;

typedef struct DotLottieConfig {
  enum Mode mode;
  bool loop_animation;
  float speed;
  bool use_frame_interpolation;
  bool autoplay;
  struct DotLottieFloatArray segment;
  uint32_t background_color;
  struct DotLottieLayout layout;
  int8_t *marker;
} DotLottieConfig;

int32_t dotlottie_active_animation_id(struct DotLottiePlayer *ptr, int8_t *result);

int32_t dotlottie_active_theme_id(struct DotLottiePlayer *ptr, int8_t *result);

int32_t dotlottie_animation_size(struct DotLottiePlayer *ptr, struct DotLottieFloatArray *result);

int32_t dotlottie_buffer_len(struct DotLottiePlayer *ptr, uint64_t *result);

int32_t dotlottie_buffer_ptr(struct DotLottiePlayer *ptr, uint64_t *result);

int32_t dotlottie_clear(struct DotLottiePlayer *ptr);

int32_t dotlottie_config(struct DotLottiePlayer *ptr, struct Config *result);

int32_t dotlottie_current_frame(struct DotLottiePlayer *ptr, float *result);

int32_t dotlottie_duration(struct DotLottiePlayer *ptr, float *result);

int32_t dotlottie_is_complete(struct DotLottiePlayer *ptr, bool *result);

int32_t dotlottie_is_loaded(struct DotLottiePlayer *ptr, bool *result);

int32_t dotlottie_is_paused(struct DotLottiePlayer *ptr, bool *result);

int32_t dotlottie_is_playing(struct DotLottiePlayer *ptr, bool *result);

int32_t dotlottie_is_stopped(struct DotLottiePlayer *ptr, bool *result);

int32_t dotlottie_load_state_machine(struct DotLottiePlayer *ptr, int8_t *str, bool *result);

int32_t dotlottie_load_state_machine_data(struct DotLottiePlayer *ptr,
                                          int8_t *state_machine_str,
                                          bool *result);

int32_t dotlottie_load_theme(struct DotLottiePlayer *ptr, int8_t *theme_id_str, bool *result);

int32_t dotlottie_load_theme_data(struct DotLottiePlayer *ptr,
                                  int8_t *theme_data_str,
                                  bool *result);

int32_t dotlottie_loop_count(struct DotLottiePlayer *ptr, uint32_t *result);

int32_t dotlottie_manifest(struct DotLottiePlayer *ptr, struct DotLottieManifest *result);

int32_t dotlottie_manifest_string(struct DotLottiePlayer *ptr, int8_t *result);

int32_t dotlottie_markers(struct DotLottiePlayer *ptr, struct DotLottieMarkerArray *result);

int32_t dotlottie_pause(struct DotLottiePlayer *ptr, bool *result);

int32_t dotlottie_play(struct DotLottiePlayer *ptr, bool *result);

int32_t dotlottie_player_load_animation(struct DotLottiePlayer *ptr,
                                        int8_t *animation_id_str,
                                        uint32_t width,
                                        uint32_t height,
                                        bool *result);

int32_t dotlottie_player_load_animation_data(struct DotLottiePlayer *ptr,
                                             int8_t *animation_data_str,
                                             uint32_t width,
                                             uint32_t height,
                                             bool *result);

int32_t dotlottie_player_load_animation_path(struct DotLottiePlayer *ptr,
                                             int8_t *animation_path_str,
                                             uint32_t width,
                                             uint32_t height,
                                             bool *result);

int32_t dotlottie_player_state_machine_subscribe(struct DotLottiePlayer *ptr,
                                                 struct StateMachineObserver *observer);

int32_t dotlottie_player_state_machine_unsubscribe(struct DotLottiePlayer *ptr,
                                                   struct StateMachineObserver *observer);

int32_t dotlottie_player_subscribe(struct DotLottiePlayer *ptr, struct Observer *observer);

int32_t dotlottie_player_unsubscribe(struct DotLottiePlayer *ptr, struct Observer *observer);

int32_t dotlottie_render(struct DotLottiePlayer *ptr, bool *result);

int32_t dotlottie_request_frame(struct DotLottiePlayer *ptr, float *result);

int32_t dotlottie_resize(struct DotLottiePlayer *ptr,
                         uint32_t width,
                         uint32_t height,
                         bool *result);

int32_t dotlottie_seek(struct DotLottiePlayer *ptr, float no, bool *result);

int32_t dotlottie_segment_duration(struct DotLottiePlayer *ptr, float *result);

int32_t dotlottie_set_frame(struct DotLottiePlayer *ptr, float no, bool *result);

int32_t dotlottie_set_state_machine_boolean_context(struct DotLottiePlayer *ptr,
                                                    int8_t *key_str,
                                                    bool value,
                                                    bool *result);

int32_t dotlottie_set_state_machine_numeric_context(struct DotLottiePlayer *ptr,
                                                    int8_t *key_str,
                                                    float value,
                                                    bool *result);

int32_t dotlottie_set_state_machine_string_context(struct DotLottiePlayer *ptr,
                                                   int8_t *key_str,
                                                   int8_t *value_str,
                                                   bool *result);

int32_t dotlottie_set_viewport(struct DotLottiePlayer *ptr,
                               int32_t x,
                               int32_t y,
                               int32_t w,
                               int32_t h,
                               bool *result);

int32_t dotlottie_start_state_machine(struct DotLottiePlayer *ptr, bool *result);

int32_t dotlottie_state_machine_framework_setup(struct DotLottiePlayer *ptr,
                                                struct ListenerType *result);

int32_t dotlottie_stop(struct DotLottiePlayer *ptr, bool *result);

int32_t dotlottie_stop_state_machine(struct DotLottiePlayer *ptr, bool *result);

int32_t dotlottie_total_frames(struct DotLottiePlayer *ptr, float *result);

struct DotLottiePlayer *new_dotlottie_player(const struct DotLottieConfig *ptr);
