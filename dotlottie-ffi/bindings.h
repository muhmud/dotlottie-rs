#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum MyEnum {
  VariantA = 0,
  VariantB = 1,
  VariantC = 2,
} MyEnum;

enum MyEnum addd(int32_t a, int32_t b);
