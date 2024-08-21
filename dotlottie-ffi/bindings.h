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
  const char *marker;
} DotLottieConfig;

struct DotLottiePlayer *new_dotlottie_player(const struct DotLottieConfig *ptr);
