#include "blinker.h"

typedef struct blinker_context_ {
    int count;
} blinker_context;

void blinker_init(blinker_config *config)
{
    static blinker_context context;
    context.count = 50;

    config->refresh_interval = 50;
    config->blinker_context = &context;
}

void blinker_tick(void *context, led_type *leds)
{
    blinker_context *c = (blinker_context*)context;

    for (int i = 0; i < led_count; ++i) {
        leds[i].r = (i + c->count) * 2;
        leds[i].g = (i + c->count) * 3;
        leds[i].b = (i + c->count) * 5;
    }
    c->count += 5;
}

void blinker_deinit(void **context)
{

}
