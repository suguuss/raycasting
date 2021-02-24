#ifndef	__PARSEMAP_H__
#define __PARSEMAP_H__

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include "raycasting.h"


typedef struct _player
{
	Point pos;
	double angle;
} Player;


typedef struct _map
{
	Player player;
	uint8_t width, height;
	uint8_t** mapArray;
} Map;

void ParseMap(char *filename, Map *map);
void PrintMap(uint8_t **map, uint8_t width, uint8_t height);
void CreateMapArray(Map *map);
void DeallocateMap(uint8_t **map, uint8_t width);
uint8_t ReadTillChar(FILE *f, char LimitChar);
#endif