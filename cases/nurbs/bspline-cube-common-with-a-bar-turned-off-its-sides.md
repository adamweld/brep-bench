+++
title = "A B-spline cube and a bar turned off its sides, from a corner, their common part"
tags = ["nurbs", "bspline", "boolean", "common", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcommon_simple/D2"

[volume]
value = 187.5
exact = "18.75·10"
tex = '18.75\cdot 10'

[area]
value = 325
exact = "2·18.75 + 10·28.75"
tex = '2\cdot 18.75 + 10\cdot 28.75'
+++

A B-spline cube and a bar turned off its sides, from a corner, their common part

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a bar from z = 0 to z = 10, its faces planes, whose footprint is the rectangle 20 long and 1.5 wide with corners (0, 0), (16, 12), (15.1, 13.2) and (−0.9, 1.2): from the corner (0, 0) its length runs 20 along (4/5, 3/5) and its width 1.5 along (−3/5, 4/5). The bar starts at the cube's corner at the origin, with its end square to its length and through that corner, and runs out through the cube's face x = 10. Make the solid common to both.

The bar's long sides are the planes 3x − 4y = 0 and 3x − 4y = −7.5, and neither meets a cube vertex other than the first corner. The common part is the prism 10 high on the parallelogram (0, 0), (10, 7.5), (10, 9.375), (0, 1.875), of area 18.75 and perimeter 2·1.875 + 2·12.5.

**As a set:**

$$[0,10]^3 \cap \{\,0 \le 4x + 3y \le 100,\ 0 \le -3x + 4y \le 7.5\,\}$$
