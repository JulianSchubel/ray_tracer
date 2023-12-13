#ifndef RT_PPM_OUTPUT_H
#define RT_PPM_OUTPUT_H

#include <fstream>
#include <iostream>
#include <ostream>

/* each pixel is written on a row
 * when read the pixels are written out left to right */
void display_ppm(const char * output_file) {
    std::ofstream ofs(output_file);
    // image config
    // P3 indicates colors are in ASCII
    std::string netpbm_magic_number = "P3";
    int image_width = 256;    
    int image_height = 256;
    int maximum_colour_value = 255;

    ofs << netpbm_magic_number << '\n' 
        << image_width << ' ' << image_height << '\n' 
        << maximum_colour_value << '\n';

    for(unsigned int y = 0; y < image_height; ++y) {
        /* get some indiction of the scanlines remaining when redirecting to a file */
        std::clog << "\rScanlines remaining: " << (image_height - (y + 1)) << std::flush; 
        for(unsigned int x = 0; x < image_width; ++x) {
            /* by convention each of the red/green/blue components are internally represented as real-valued variables from 0.0 to 1.0 */
            double r = double(x) / (image_width - 1);
            double g = double(y) / (image_height - 1);
            double b = 0;

            /* scale real-valued red/green/blue component values to integers */
            int xr = static_cast<int>(255.99 * r);
            int xg =  static_cast<int>(255.99 * g);
            int xb = static_cast<int>(255.99 * b);

            ofs << '\r' << xr << ' ' << xg << ' ' << xb << ' ' << '\n';
         }
    }
    std::clog << "\rDone.                           \n";
}

#endif //RT_PPM_OUTPUT_H
