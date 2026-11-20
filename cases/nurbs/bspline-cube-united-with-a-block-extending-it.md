+++
title = "A B-spline cube united with a block that carries it twice as high"
tags = ["nurbs", "bspline", "boolean", "union", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bfuse_simple/A7, OCCT boolean/bfuse_simple/B4"

[volume]
value = 2000
exact = "10 · 10 · 20"
tex = '10\cdot 10\cdot 20'

[area]
value = 1000
exact = "2·10² + 4·10·20"
tex = '2\cdot 10^2 + 4\cdot 10\cdot 20'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 0) to (10, 10, 20), its faces planes. Unite them. The block's four sides lie in the cube's four sides and its bottom in the cube's bottom, and it is taller than the cube: the union is the block, a box 10 by 10 by 20, its four sides each one flat face where the two operands' faces met.

**As a set:**

$$[0,10]^3 \cup [0,10]\times[0,10]\times[0,20]$$
