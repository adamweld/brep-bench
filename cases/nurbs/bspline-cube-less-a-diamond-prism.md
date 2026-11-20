+++
title = "A B-spline cube less a diamond prism"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/F3"

[volume]
value = 500.0
exact = "half the cube, ½·10³"
tex = '\tfrac12\cdot 10^3'

[area]
value = 441.4213562373095
exact = "2·50 + 10·(10 + 10 + 10√2)"
tex = '2\cdot 50 + 10\,(10 + 10 + 10\sqrt2)'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a prism from z = 0 to z = 10 on the square with corners (0, 0), (10, 10), (20, 0) and (10, −10), its faces planes. Cut the prism from the cube.

The square is turned 45° to the cube. Two of its corners, (0, 0) and (10, 10), are corners of the cube's bottom, so its side between them runs along the bottom's diagonal; its diagonal from (0, 0) to (20, 0) runs along the cube's bottom edge on the x axis and on past it. The prism and the cube share the planes z = 0 and z = 10. The cut takes away the half of the cube below the diagonal plane, and what is left is the prism 10 high on the triangle with corners (0, 0), (10, 10) and (0, 10): its two square sides lie in the cube's faces x = 0 and y = 10, its triangles in the cube's top and bottom, and its slanting side, through the cube's diagonal, is the prism's.

**As a set:**

$$[0,10]^3 \setminus \{\,|x - 10| + |y| \le 10,\ 0 \le z \le 10\,\}$$
