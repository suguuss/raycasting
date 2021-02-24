#ifndef __GRAPHICS_H__
#define __GRAPHICS_H__

#include <stdint.h>
#include "gfx.h"
#include "raycasting.h"
#include "parsemap.h"

#define MOVE_SPEED 0.5

int CreateWindow(Map *map, uint16_t width, uint16_t height);

#endif