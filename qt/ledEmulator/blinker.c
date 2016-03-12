#include "blinker.h"

led_type image[3][led_count] = {
    {
        {180, 0, 0},
        {190, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {240, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 190, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {100, 200, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 155},
    },
    {
        {255, 255, 0},
        {255, 255, 0},
        {0, 255, 0},
        {0, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 0, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
        {255, 255, 0},
    },
    {
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {255, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {127, 0, 127},
        {0, 0, 127},
        {0, 0, 64},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 255},
        {0, 0, 155},
    },
};

static struct {
    int line;
} blinker_context;

void blinker_init(blinker_config *config)
{
    config->refresh_interval = 250;
}

void blinker_tick(led_type *leds)
{
    for (int i = 0; i < led_count; ++i) {
        leds[i] = image[blinker_context.line][i];
    }
    blinker_context.line = (blinker_context.line + 1) % 3;
}

void blinker_deinit(blinker_config *config)
{

}
