+++
title = "An oblique pyramid lofted from a triangle to a point"
tags = ["loft", "ruled", "point-end", "pyramid", "capped", "analytic"]
origin = "OCCT thrusection/specific/H6"

[volume]
value = 34.64101615137755
exact = "(1/3)·A·h with A = 12√3 the triangle's area and h = 5 the apex's height, whatever its sideways position"
tex = '\tfrac13\cdot 12\sqrt3\cdot 5 = 20\sqrt3'

[area]
value = 77.26393854270013
exact = "the triangle 12√3, plus the three slanted triangles, each half the length of the cross product of two of its edges: √(1317/4 − 9√3), √408 and √(1317/4 + 9√3)"
tex = '12\sqrt3 + \sqrt{\tfrac{1317}{4} - 9\sqrt3} + \sqrt{408} + \sqrt{\tfrac{1317}{4} + 9\sqrt3}'
+++

A three-sided pyramid whose apex is not over the middle of its base, as a ruled loft from a triangle to a single point, the solid closed over the triangle by a flat face.

The triangle lies in the plane z = 0, with corners (4, 0, 0), (−2, 2√3, 0) and (−2, −2√3, 0): equilateral, side 4√3, centered on the z axis. The point is (1, 0.5, 5), above the inside of the triangle and off its axis. The point is joined to every point of the triangle's boundary by a straight line, so the solid has four flat faces: the triangle and three slanted triangles rising from its sides to the point.

**As a set:**

$$\operatorname{conv}\bigl((4,0,0),\ (-2,\pm2\sqrt3,0),\ (1,0.5,5)\bigr)$$
