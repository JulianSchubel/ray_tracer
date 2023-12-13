#include "./vec3.h"

#ifndef VEC3_DEFINITION
#define VEC3_DEFINITION

#include <cmath>
#include <cstddef>
#include <ostream>

/* constructors */
vec3::vec3() : e{0,0,0} {

}

vec3::vec3(double e1, double e2, double e3) : e{e1,e2,e3} {}

vec3::vec3(const vec3& v) {
    e[0] = v[0];
    e[1] = v[1];
    e[2] = v[2];
}

/* destructor */
vec3::~vec3() {}

/* accessors */
double vec3::x() const {
    return this->e[0];
}

double vec3::y() const {
    return this->e[1];
}

double vec3::z() const {
    return this->e[2];
}

/* overloaded operators */
vec3 vec3::operator -() const {
    return vec3(-e[0], -e[1], -e[2]);
}

vec3& vec3::operator =(const vec3& v) {
    if(this != &v) {
        e[0] = v[0];
        e[1] = v[1];
        e[2] = v[2];
    }
    return *this;
}

vec3& vec3::operator +=(const vec3& v) {
    e[0] += v[0]; 
    e[1] += v[1];
    e[2] += v[2];
    return *this;
}

vec3& vec3::operator *=(const vec3& v) {
    e[0] *= v[0];
    e[1] *= v[1];
    e[2] *= v[2];
    return *this;
}

vec3& vec3::operator *=(double t) {
    for(double &x : e) {
        x *= t;
    }
    return *this;
}

vec3& vec3::operator /=(const vec3& v) {
    e[0] /= v[0];
    e[1] /= v[1];
    e[2] /= v[2];
    return *this;
}

vec3& vec3::operator /=(double t) {
    return *this *= 1/t;
}

double vec3::operator[](std::size_t i) const {
    return e[i];
}

double& vec3::operator[](std::size_t i) {
    return e[i];
}

/* public methods */
double vec3::length_squared() const {
    double sum = 0;
    for(double x : e) {
        sum += x*x;
    }
    return sum;
}

double vec3::length() const {
    return std::sqrt(this->length_squared());
}

#endif //VEC3_DEFINITION
