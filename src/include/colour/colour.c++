#ifndef COLOUR_DEFINITION_H
#define COLOUR_DEFINITION_H

#include "./colour.h"
 
void write_colour(std::ostream &out, colour pixel_colour) {
   out << static_cast<int>(scalar * pixel_colour.x()) << ' '
       << static_cast<int>(scalar * pixel_colour.y()) << ' '
       << static_cast<int>(scalar * pixel_colour.z()) << '\n';
}

#endif //COLOUR_DEFINITION_H
