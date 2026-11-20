+++
title = "A cylinder with two flats"
tags = ["boolean", "cut", "analytic", "cylinder", "planar"]
origin = "OCCT boolean/volumemaker/E1, OCCT boolean/volumemaker/H5, OCCT boolean/volumemaker/H6, OCCT boolean/volumemaker/I1"

[volume]
value = 449.40044351731375
exact = "8·(24 + 50·asin(3/5)): the section, the disc of radius 5 within |x| ≤ 3, has area 2∫₋₃³√(25 − x²)dx"
tex = '8\left(24 + 50\arcsin\tfrac35\right)'

[area]
value = 343.31028828625394
exact = "2·(24 + 50·asin(3/5)) + 2·(8·8) + 8·5·4·asin(3/5)"
tex = '2\left(24 + 50\arcsin\tfrac35\right) + 2\cdot 8\cdot 8 + 8\cdot 5\cdot 4\arcsin\tfrac35'
+++

A cylinder of radius 5 and height 8, its axis the z axis, from z = 0 to z = 8, with a flat milled along each side: the planes x = 3 and x = −3. What is left is the part of the cylinder where |x| ≤ 3, bounded by four planes (the two ends and the two flats) and two strips of the cylinder's side, each 73.74° of its turn wide, on the +y and −y sides.

**As a set:**

$$\{\,\rho \le 5,\ |x| \le 3,\ 0 \le z \le 8\,\}$$
