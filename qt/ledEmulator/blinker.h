#ifndef BLINKER_H
#define BLINKER_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

enum globals_ {
    led_count = 30
};

typedef struct blinker_config_ {
    int refresh_interval; // in milliseconds
    void *blinker_context;
} blinker_config;

typedef struct led_ {
    uint8_t r;
    uint8_t g;
    uint8_t b;
} led_type;

void blinker_init(blinker_config *config);
void blinker_tick(void *context, led_type *leds);
void blinker_deinit(void **context);

#ifdef __cplusplus
}
#endif

#endif // BLINKER_H
