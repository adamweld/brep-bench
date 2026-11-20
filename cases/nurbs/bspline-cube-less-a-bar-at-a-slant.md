+++
title = "A B-spline cube less a bar turned off its sides, from a corner"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/F7"

[volume]
value = 812.5
exact = "(37.5 + 43.75)·10"
tex = '(37.5 + 43.75)\cdot 10'

[area]
value = 775.0
exact = "2·81.25 + 10·(30 + 31.25)"
tex = '2\cdot 81.25 + 10\,(30 + 31.25)'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a bar from z = 0 to z = 10, its faces planes, whose footprint is the rectangle 20 long and 1.5 wide with corners (0, 0), (16, 12), (15.1, 13.2) and (−0.9, 1.2): from the corner (0, 0) its length runs 20 along (4/5, 3/5) and its width 1.5 along (−3/5, 4/5). The bar starts at the cube's corner at the origin, with its end square to its length and through that corner, and runs out through the cube's face x = 10. Cut the bar from the cube.

The bar's long sides are the planes 4y = 3x and 4y = 3x + 7.5 (that is, 3x − 4y = 0 and 3x − 4y = −7.5), and neither meets a cube vertex other than the first corner; no face of the bar is in a face of the cube except its bottom and top, which lie in the cube's. The cut leaves two separate solids, each 10 high: one on the triangle (0, 0), (10, 0), (10, 7.5) and one on the quadrilateral (0, 1.875), (10, 9.375), (10, 10), (0, 10).

**As a set:**

$$[0,10]^3 \setminus \{\,0 \le 4x + 3y \le 100,\ 0 \le -3x + 4y \le 7.5,\ 0 \le z \le 10\,\}$$
