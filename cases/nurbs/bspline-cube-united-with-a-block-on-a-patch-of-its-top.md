+++
title = "A B-spline cube united with a block standing on a patch of its top"
tags = ["nurbs", "bspline", "boolean", "union", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bfuse_simple/B2, OCCT boolean/bfuse_simple/B7"

[volume]
value = 1125
exact = "1000 + 5·5·5"
tex = '1000 + 5\cdot 5\cdot 5'

[area]
value = 700
exact = "6·10² + 4·5·5: the block covers 5·5 of the top and adds its own 5·5 top, so only its four sides add"
tex = '6\cdot 10^2 + 4\cdot 5\cdot 5'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 10) to (5, 5, 15), its faces planes. Unite them. The block stands on the cube's top face at one corner, its bottom a 5 by 5 patch of that face and its two outer sides in the planes of the cube's x = 0 and y = 0 faces; no volume overlaps. The union is the cube with a 5 by 5 by 5 step on one corner of its top.

**As a set:**

$$[0,10]^3 \cup [0,5]\times[0,5]\times[10,15]$$
