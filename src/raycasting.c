#include "raycasting.h"

double GetWallDistance(Point Ip, double angle, uint8_t** map)
{
	Point p = {Ip.x, Ip.y};

	// Values X
	double hypSide, opposeSide, adjacentSide;
	// Values Y
	double hypTop, opposeTop, adjacentTop;

	uint8_t isWall = 0;
	
	if(angle == floor(angle)) angle -= 0.00001;

	while (1)
	{
		switch (INT(angle) / 90)
		{
			case 0: // 0-89°
				adjacentSide = GetClosestValueUp(p.x) - p.x; // X+
				opposeTop = p.y - GetClosestValueDown(p.y); // Y-
				break;
			case 1: // 90-179°
				adjacentSide = p.x - GetClosestValueDown(p.x); // X-
				opposeTop = p.y - GetClosestValueDown(p.y); // Y-
				break;
			case 2: // 180-269°
				adjacentSide = p.x - GetClosestValueDown(p.x); // X-
				opposeTop = GetClosestValueUp(p.y) - p.y; // Y+
				break;
			case 3: // 270-359°
				adjacentSide = GetClosestValueUp(p.x) - p.x; // X+
				opposeTop = GetClosestValueUp(p.y) - p.y; // Y+
				break;
		}

		// X
		// Calculer la distance avec la case la plus proche
		// adjacentSide = GetClosestValueUp(p.x) - p.x;
		// Calculer la taille de l'opposé
		// x = tan(angle) * adjacent(adjacent)
		opposeSide = tan(DEG2RAD(angle)) * adjacentSide; 
		// Calculer l'hypothenuse
		hypSide = sqrt(adjacentSide*adjacentSide + opposeSide*opposeSide);

		// Y
		// Calculer la distance avec la case la plus proche
		// opposeTop = p.y - GetClosestValueDown(p.y);
		// Calculer la taille de l'adjacent
		// adjacent = oppose / tan(angle)
		adjacentTop = opposeTop / tan(DEG2RAD(angle)); 
		// Calculer l'hypothenuse
		hypTop = sqrt(adjacentTop*adjacentTop + opposeTop*opposeTop);

		if (hypSide < hypTop) 
		{
			// printf("Collision with wall on right\n");
			// Calculate new position
			// p.x = GetClosestValueUp(p.x);
			// p.y = p.y - opposeSide;

			// is there a wall
			// isWall = map[INT(p.x)][INT(p.y)];
			// printf("AAAAA\n");

			if (opposeSide < 0.0) opposeSide *= -1;

			switch (INT(angle) / 90)
			{
				case 0: // 0-89°
					p.x = GetClosestValueUp(p.x);
					p.y -= opposeSide;
					isWall = map[INT(p.x)][INT(p.y)];
					break;
				case 1: // 90-179°
					p.x = GetClosestValueDown(p.x);
					p.y -= opposeSide;
					isWall = map[INT(p.x)-1][INT(p.y)];
					break;
				case 2: // 180-269°
					p.x = GetClosestValueDown(p.x);
					p.y += opposeSide;
					isWall = map[INT(p.x)-1][INT(p.y)];
					break;
				case 3: // 270-359°
					p.x = GetClosestValueUp(p.x);
					p.y += opposeSide;
					isWall = map[INT(p.x)][INT(p.y)];
					break;
			}
		}
		else 
		{
			// printf("Collision with wall on top\n");
			// Calculate new position
			// p.x += adjacentTop;
			// p.y = GetClosestValueDown(p.y);
			// printf("BBBBBB\n");
			// is there a wall
			// isWall = map[INT(p.x)][INT(p.y)-1];

			if (adjacentTop < 0.0) adjacentTop *= -1;

			switch (INT(angle) / 90)
			{
				case 0: // 0-89°
					p.x += adjacentTop;
					p.y = GetClosestValueDown(p.y);
					isWall = map[INT(p.x)][INT(p.y)-1];
					break;
				case 1: // 90-179°
					// printf("x = %f | y = %f | iswall = %d.\n", p.x, p.y, isWall);
					p.x -= adjacentTop;
					p.y = GetClosestValueDown(p.y);
					isWall = map[INT(p.x)][INT(p.y)-1];
					// printf("x = %f | y = %f | iswall = %d\n", p.x, p.y, isWall);
					break;
				case 2: // 180-269°
					p.x -= adjacentTop;
					p.y = GetClosestValueUp(p.y);
					isWall = map[INT(p.x)][INT(p.y)];
					break;
				case 3: // 270-359°
					p.x += adjacentTop;
					p.y = GetClosestValueUp(p.y);
					isWall = map[INT(p.x)][INT(p.y)];
					break;
			}
		}

		if (isWall == 1) break;
	}



	//hyp between the initial point and the point on the wall
	// (P.x - I.x)**2 + (I.y - P.y)*2
	return sqrt( pow(p.x - Ip.x, 2) + pow(Ip.y - p.y, 2));
	// return 0;
}

double GetClosestValueUp(double x)
{
	return (int)(x + 1);
}

double GetClosestValueDown(double y)
{
	if (floor(y) == y)
	{
		return y - 1;
	}
	else
	{
		return floor(y);
	}
}