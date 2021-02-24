#include "graphics.h"


void render(struct gfx_context_t *ctxt, Map *map, uint16_t width, uint16_t height)
{
	double angleStep = 90.0/width;
	double wallDistance = 0;
	uint16_t wallHeight = 0;
	uint16_t fromTop = 0;

	// Dessine le ciel et le sol
	for (uint16_t x = 0; x < width; x++)
	{
		for (uint16_t y = 0; y < height; y++)
		{
			if (y < height/2) {gfx_putpixel(ctxt, x, y, COLOR_BLUE);}
			else {gfx_putpixel(ctxt, x, y, COLOR_BLACK);}
		}
	}
	
	// // Dessine les murs
	for (uint16_t x = 0; x <= width; x++)
	{
		// Get the wall distance in cm
		wallDistance = GetWallDistance(map->player.pos, angleStep * x+map->player.angle/2, map->mapArray) * 100.0;
		// Get the wall height
		if (wallDistance >= 150)
		{
			wallHeight = height - (wallDistance - 150) / 10 * 5;
			if (wallHeight < 10 || wallHeight > height) wallHeight = 10;
		}

		// Savoir a combien du haut et du bas le mur est
		fromTop = (height - wallHeight) / 2;

		for (uint16_t y = fromTop; y < wallHeight+fromTop; y++)
		{
			if (wallDistance > 325)
			{
				gfx_putpixel(ctxt, width-x, y, MAKE_COLOR(0xA0, 0, 0));
				// gfx_putpixel(ctxt, x, y, MAKE_COLOR(0xA0, 0, 0));
			}
			else
			{
				gfx_putpixel(ctxt, width-x, y, COLOR_RED);	
			}
		}

		// printf("WallDistance : %f | WallHeight : %d | FromTop : %d | angle : %f | x : %d\n", wallDistance, wallHeight, fromTop, x*angleStep, x);
	}

}


/// Read the file until there is a blank character
/// @param width Width of the window to create
/// @param height Height of the window to create
int CreateWindow(Map *map, uint16_t width, uint16_t height)
{
	struct gfx_context_t *ctxt = gfx_create("Image Display", width, height);
	// double angleStep = 90.0/(width/2.0);
	uint32_t key = 0;
	uint32_t ticks = 0;

	if (!ctxt) {
			fprintf(stderr, "Graphics initialization failed!\n");
			return EXIT_FAILURE;
	}
	



	while (key != SDLK_ESCAPE) {
		render(ctxt, map, width, height);
		gfx_present(ctxt);

		key = gfx_keypressed();
		switch (key)
		{
		case SDLK_RIGHT:
			if (map->mapArray[INT(map->player.pos.x + MOVE_SPEED)][INT(map->player.pos.y)] != 1) {map->player.pos.x += MOVE_SPEED;}
			break;
		case SDLK_LEFT:
			if (map->mapArray[INT(map->player.pos.x - MOVE_SPEED)][INT(map->player.pos.y)] != 1) {map->player.pos.x -= MOVE_SPEED;}
			break;
		case SDLK_UP:
			if (map->mapArray[INT(map->player.pos.x)][INT(map->player.pos.y - MOVE_SPEED)] != 1) {map->player.pos.y -= MOVE_SPEED;}
			break;
		case SDLK_DOWN:
			if (map->mapArray[INT(map->player.pos.x)][INT(map->player.pos.y + MOVE_SPEED)] != 1) {map->player.pos.y += MOVE_SPEED;}
			break;
		default:
			break;
		}

		printf("\r%d", SDL_GetTicks() - ticks);
		ticks = SDL_GetTicks();
	}

	gfx_destroy(ctxt);
	return EXIT_SUCCESS;
}