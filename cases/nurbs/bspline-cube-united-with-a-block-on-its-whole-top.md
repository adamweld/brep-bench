+++
title = "A B-spline cube united with a block standing on its whole top"
tags = ["nurbs", "bspline", "boolean", "union", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bfuse_simple/A8"

[volume]
value = 1500
exact = "10 · 10 · 15"
tex = '10\cdot 10\cdot 15'

[area]
value = 800
exact = "2·10² + 4·10·15"
tex = '2\cdot 10^2 + 4\cdot 10\cdot 15'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 10) to (10, 10, 15), its faces planes. Unite them. The block's bottom face is the cube's top face and no volume overlaps: the union is a box 10 by 10 by 15, its sides each one flat face where the two operands' faces met.

**As a set:**

$$[0,10]^3 \cup [0,10]\times[0,10]\times[10,15]$$
