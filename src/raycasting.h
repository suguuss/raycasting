#ifndef __RAYCASTING_H__
#define __RAYCASTING_H__

#include <inttypes.h>
#include <stdio.h>
#include <math.h>

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

#define DEG2RAD(X) X*M_PI/180.0
#define INT(X) (int)(X)


typedef struct _point
{
	double x, y;
} Point;


double GetWallDistance(Point p, double angle, uint8_t** map);
double GetClosestValueUp(double x);
double GetClosestValueDown(double y);


#endif