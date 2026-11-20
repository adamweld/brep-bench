+++
title = "A block less a B-spline cube it overhangs at one end"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/A9"

[volume]
value = 1000
exact = "10 × 10 × (20 − 10)"
tex = '10\cdot 10\cdot(20-10)'

[area]
value = 600
exact = "6 · 10²"
tex = '6\cdot 10^2'
+++

A block from (0, 0, 0) to (10, 10, 20), its faces planes. And a cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. Cut the cube from the block. The block's four sides lie in the cube's four sides and its bottom in the cube's bottom, and it runs on above the cube's top by the cube's own height. What is left is the cube-sized block above the cube, from (0, 0, 10) to (10, 10, 20).

**As a set:**

$$([0,10]\times[0,10]\times[0,20]) \setminus [0,10]^3$$
