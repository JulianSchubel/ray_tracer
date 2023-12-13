#ifndef RT_COLOUR_H
#define RT_COLOUR_H

#include "../vec3/vec3.h"
#include <iostream>

/* alias for vec3 */
using colour = vec3;

constexpr double scalar = 255.999;
void write_colour(std::ostream &out, colour pixel_colour);

#endif //RT_COLOUR_H
