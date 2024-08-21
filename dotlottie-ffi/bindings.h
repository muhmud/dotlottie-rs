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

typedef struct DotLottieFloatData {
  float *ptr;
  size_t size;
} DotLottieFloatData;

typedef struct DotLottieLayout {
  enum Fit fit;
  struct DotLottieFloatData align;
} DotLottieLayout;

typedef struct DotLottieConfig {
  enum Mode mode;
  bool loop_animation;
  float speed;
  bool use_frame_interpolation;
  bool autoplay;
  struct DotLottieFloatData segment;
  uint32_t background_color;
  struct DotLottieLayout layout;
  int8_t *marker;
} DotLottieConfig;

bool dotlottie_player_load_animation_data(struct DotLottiePlayer *ptr,
                                          int8_t *animation_data_str,
                                          uint32_t width,
                                          uint32_t height);

struct DotLottiePlayer *new_dotlottie_player(const struct DotLottieConfig *ptr);
