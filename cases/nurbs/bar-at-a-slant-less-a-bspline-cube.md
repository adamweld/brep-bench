+++
title = "A bar turned off the sides of a B-spline cube, less the cube"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/F8"

[volume]
value = 112.5
exact = "(20·1.5 − 18.75)·10"
tex = '(20\cdot 1.5 - 18.75)\cdot 10'

[area]
value = 240.0
exact = "2·11.25 + 10·(4.5 + 17.25)"
tex = '2\cdot 11.25 + 10\,(4.5 + 17.25)'
+++

A bar from z = 0 to z = 10, its faces planes, whose footprint is the rectangle 20 long and 1.5 wide with corners (0, 0), (16, 12), (15.1, 13.2) and (−0.9, 1.2): from the corner (0, 0) its length runs 20 along (4/5, 3/5) and its width 1.5 along (−3/5, 4/5). And a cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. The bar starts at the cube's corner at the origin, with its end square to its length and through that corner, and runs out through the cube's face x = 10. Cut the cube from the bar.

The bar's long sides are the planes 4y = 3x and 4y = 3x + 7.5. The cube takes out of the bar's footprint, which has area 30, the part of it between x = 0 and x = 10, of area 18.75. What is left is two separate solids, each 10 high: a small triangle (0, 0), (−0.9, 1.2), (0, 1.875) outside the cube's face x = 0, which touches the cube's face there along a strip, and the quadrilateral (10, 7.5), (16, 12), (15.1, 13.2), (10, 9.375) beyond the face x = 10.

**As a set:**

$$\{\,0 \le 4x + 3y \le 100,\ 0 \le -3x + 4y \le 7.5,\ 0 \le z \le 10\,\} \setminus [0,10]^3$$
