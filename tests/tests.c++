#include <catch2/catch_all.hpp>
#include "../src/include/vec3/vec3.h"
unsigned int Factorial( unsigned int number ) {
    return number <= 1 ? number : Factorial(number-1)*number;
}

vec3 v(0,1,2);

TEST_CASE( "Vec3 components are accessible", "[Vec3]" ) {
    REQUIRE( v.x() == 0 );
    REQUIRE( v.y() == 1 );
    REQUIRE( v.z() == 2 );
}
