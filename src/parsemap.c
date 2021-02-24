#include "parsemap.h"

void ParseMap(char *filename, Map *map)
{
	FILE *f = fopen(filename, "r");
	char caractere = 0;
	if (f != NULL)
	{
		// Reads the header of the file
		map->height = ReadTillChar(f, ',');
		map->width = ReadTillChar(f, ',');
		map->player.angle = (double)(ReadTillChar(f, ','));

		CreateMapArray(map);

		// Populating the array with the file content
		for (uint8_t y = 0; y < map->height; y++) 
		{
			for (uint8_t x = 0; x < map->width; x++)
			{
				caractere = 0;
				
				do // Get rid of the \n\r in the file
				{
					fread(&caractere, 1, 1, f);
					caractere = caractere - 0x30;
				} while (caractere != 0 && caractere != 1 && caractere != 2);
				map->mapArray[x][y] = caractere;

				if (caractere == 2)
				{
					map->player.pos.x = x + 0.5;
					map->player.pos.y = y + 0.5;
				}
			}
		}
		
		fclose(f);
	}
}


void PrintMap(uint8_t **map, uint8_t width, uint8_t height)
{
	for (uint8_t y = 0; y < height; y++) //5 
	{
		for (uint8_t x = 0; x < width; x++) //4
		{
			printf("%d", map[x][y]);
		}
		printf("\n");
	}
}

void CreateMapArray(Map *map)
{
	map->mapArray = malloc(map->width * sizeof(uint8_t*));
	for (uint8_t index; index < map->width; index++)
	{
		map->mapArray[index] = malloc(map->height * sizeof(uint8_t));
	}
}

void DeallocateMap(uint8_t **map, uint8_t width)
{
	for (uint8_t x = 0; x < width; x++)
	{
		free(map[x]);
	}

	free(map);
}

uint8_t ReadTillChar(FILE *f, char LimitChar)
{
	char caractere = 0;
	uint8_t out = 0;
	while (caractere != LimitChar)
	{
		fread(&caractere, 1, 1, f);
		if (caractere == LimitChar) break;
		out = out * 10 + caractere - 0x30;
	}
	return out;
}