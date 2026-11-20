+++
title = "A block less a B-spline cube it overhangs, shifted up"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/C3"

[volume]
value = 500
exact = "10 × 10 × (15 − 10)"
tex = '10\cdot 10\cdot 5'

[area]
value = 400
exact = "2·10² + 4·10·5"
tex = '2\cdot 10^2 + 4\cdot 10\cdot 5'
+++

A block from (0, 0, 5) to (10, 10, 15), its faces planes. And a cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. Cut the cube from the block. The block's four sides lie in the cube's four sides; its bottom is inside the cube, halfway up, and its top is above the cube's top. What is left is the part of the block above the cube, from (0, 0, 10) to (10, 10, 15).

**As a set:**

$$([0,10]\times[0,10]\times[5,15]) \setminus [0,10]^3$$
