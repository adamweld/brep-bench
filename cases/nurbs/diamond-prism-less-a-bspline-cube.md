+++
title = "A diamond prism less a B-spline cube"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/F4"

[volume]
value = 1500.0
exact = "(4·10² − 50)·10"
tex = '(2\cdot 10^2 - 50)\cdot 10'

[area]
value = 924.2640687119285
exact = "2·150 + 10·(20 + 30√2)"
tex = '2\cdot 150 + 10\,(20 + 30\sqrt2)'
+++

A prism from z = 0 to z = 10 on the square with corners (0, 0), (10, 10), (20, 0) and (10, −10), its faces planes. And a cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. Cut the cube from the prism.

The square is turned 45° to the cube. Two of its corners, (0, 0) and (10, 10), are corners of the cube's bottom, so its side between them runs along the bottom's diagonal; its diagonal from (0, 0) to (20, 0) runs along the cube's bottom edge on the x axis and on past it. The prism and the cube share the planes z = 0 and z = 10. The cube takes the triangle with corners (0, 0), (10, 0) and (10, 10) out of the prism's footprint, which has area 200, and what is left is the prism 10 high on the five-cornered footprint (0, 0), (10, −10), (20, 0), (10, 10), (10, 0), of area 150.

**As a set:**

$$\{\,|x - 10| + |y| \le 10,\ 0 \le z \le 10\,\} \setminus [0,10]^3$$
