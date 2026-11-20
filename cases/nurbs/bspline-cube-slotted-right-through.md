+++
title = "A B-spline cube slotted right through by a block"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/C4"

[volume]
value = 600
exact = "10·10·(10 − 4)"
tex = '10\cdot 10\cdot(10-4)'

[area]
value = 640
exact = "2·(2·10² + 4·10·3)"
tex = '2\left(2\cdot 10^2 + 4\cdot 10\cdot 3\right)'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 3) to (10, 10, 7), its faces planes. Cut the block from the cube. The block's four sides lie in the cube's four sides, and its bottom and top are inside the cube, at heights 3 and 7. It cuts a slot through the whole width of the cube, and what is left is two separate slabs 3 thick: the box from (0, 0, 0) to (10, 10, 3) and the box from (0, 0, 7) to (10, 10, 10).

**As a set:**

$$[0,10]^3 \setminus ([0,10]\times[0,10]\times[3,7])$$
