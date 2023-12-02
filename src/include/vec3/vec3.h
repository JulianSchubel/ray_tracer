#ifndef RT_VEC3_H
#define RT_VEC3_H

#include <cmath>
#include <cstddef>
#include <iostream>

class vec3 {
    public:
        /* constructors */
        vec3();
        vec3(double e1, double e2, double e3);
        vec3(const vec3&);
        /* destructor */
        ~vec3();

        /* accessors */
        double x() const;
        double y() const;
        double z() const;

        /* overloaded operators */
        vec3 operator -() const;

        vec3& operator =(const vec3&);
        vec3& operator +=(const vec3&);
        vec3& operator *=(const vec3&);
        vec3& operator *=(double t);
        vec3& operator /=(const vec3&);
        vec3& operator /=(double t);

        friend inline std::ostream& operator <<(std::ostream &out, const vec3&);
        friend inline std::ostream& operator<<(std::ostream& out, const vec3& v);
        friend inline vec3 operator +(const vec3& u, const vec3& v);
        friend inline vec3 operator -(const vec3& u, const vec3& v);
        friend inline vec3 operator *(const vec3& u, const vec3& v);
        friend inline vec3 operator *(double t, const vec3& v);
        friend inline vec3 operator *(const vec3& v, double t) ;
        friend inline vec3 operator / (const vec3& v, double t);
        friend inline double dot_product(const vec3& u, const vec3& v);
        friend inline vec3 cross_product(const vec3& u, const vec3& v);
        friend inline vec3 unit_vector(const vec3& v);

        /* const vec3 objects return copy of e[i] */
        double operator[](std::size_t i) const ;

        /* non-const vec3 object returns reference to e[i] */
        double& operator[](std::size_t i);

        double length_squared() const;
        double length() const;
    private:
        double e[3];
};

#ifndef VEC3_DEFINITION
#include "./vec3.c++"
#endif //VEC3_DEFINITION
#endif //RT_VEC3_H
