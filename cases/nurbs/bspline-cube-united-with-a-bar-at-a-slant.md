+++
title = "A B-spline cube united with a bar turned off its sides, from a corner"
tags = ["nurbs", "bspline", "boolean", "union", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bfuse_simple/D2"

[volume]
value = 1112.5
exact = "10³ + 20·1.5·10 − 187.5"
tex = '10^3 + 20\cdot 1.5\cdot 10 - 187.5'

[area]
value = 765
exact = "2·(100 + 30 − 18.75) + 10·54.25"
tex = '2\,(100 + 30 - 18.75) + 10\cdot 54.25'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a bar from z = 0 to z = 10, its faces planes, whose footprint is the rectangle 20 long and 1.5 wide with corners (0, 0), (16, 12), (15.1, 13.2) and (−0.9, 1.2): from the corner (0, 0) its length runs 20 along (4/5, 3/5) and its width 1.5 along (−3/5, 4/5). The bar starts at the cube's corner at the origin, with its end square to its length and through that corner, and runs out through the cube's face x = 10. Unite them.

The bar's long sides are the planes 3x − 4y = 0 and 3x − 4y = −7.5, and neither meets a cube vertex other than the first corner. The two solids share the planes z = 0 and z = 10. The union is 10 high on a polygon of ten sides: the square with the bar's footprint added, which pokes out past the face x = 0 at the start, near the corner, and past the face x = 10 at the far end.

**As a set:**

$$[0,10]^3 \cup \{\,0 \le 4x + 3y \le 100,\ 0 \le -3x + 4y \le 7.5,\ 0 \le z \le 10\,\}$$
