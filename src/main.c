#include "parsemap.h"
#include "raycasting.h"
#include "graphics.h"

#define MAP_WIDTH 16
#define MAP_HEIGHT 16
#define WINDOW_WIDTH 1024
#define WINDOW_HEIGHT 576



int main()
{
	// LOCAL VARIABLES 
	Map map;

	// START MAIN
	ParseMap("map3.txt", &map);
	PrintMap(map.mapArray, map.width, map.height);
	// printf("%f | %f\n", map.player.pos.x, map.player.pos.y);

	CreateWindow(&map, WINDOW_WIDTH, WINDOW_HEIGHT);

	DeallocateMap(map.mapArray, map.width);
	return 0;
}