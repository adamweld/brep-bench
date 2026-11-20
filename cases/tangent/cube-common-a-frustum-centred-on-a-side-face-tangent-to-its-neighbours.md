+++
title = "A cube and a frustum centred on a side face, tangent to the faces beside it, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cone", "planar"]
origin = "OCCT tests/boolean/bopcommon_simple/ZG5"

[volume]
value = 229.07446432425576
exact = "half of πh/3·(a² + ab + b²), h = 10, a = 5, b = 5/2"
tex = '\tfrac{1}{2}\cdot\tfrac{\pi h}{3}(a^2 + ab + b^2),\quad h = 10,\ a = 5,\ b = \tfrac52'

[area]
value = 245.52286968185794
exact = "π(a² + b²)/2 + π(a + b)·√(h² + (a − b)²)/2 + (a + b)·h, the last the trapezoid in the plane y = 0"
tex = '\tfrac{\pi}{2}(a^2+b^2) + \tfrac{\pi}{2}(a+b)\sqrt{h^2+(a-b)^2} + (a+b)h'
+++

A cube of edge 10: x from −5 to 5, y from 0 to 10, z from 0 to 10. A frustum of a cone with its axis the vertical line through (0, 0), from z = 0 to z = 10, of radius 5 at the bottom and 5/2 at the top. Its axis lies in the cube's side face y = 0, at the middle of it, and the frustum is as tall as the cube, its ends level with the cube's bottom and top faces. Take the part common to the two.

The frustum's base circle has radius 5, half the face's width, so it is tangent to the cube's faces x = −5 and x = 5 at (±5, 0, 0), and the cone narrows from there, staying inside the cube's width. Half the frustum (y ≥ 0) lies inside the cube; the other half lies outside.

What is left is the half of the frustum with y ≥ 0: a half cone wall, two half discs and a flat trapezoid in the cube's face y = 0.

**As a set:**

$$\bigl([-5,5]\times[0,10]\times[0,10]\bigr) \cap \{\,\rho \le 5 - \tfrac{z}{4},\ 0 \le z \le 10\,\}$$
