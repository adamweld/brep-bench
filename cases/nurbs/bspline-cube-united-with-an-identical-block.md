+++
title = "A B-spline cube united with a block that is the same cube"
tags = ["nurbs", "bspline", "boolean", "union", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bfuse_simple/A6"

[volume]
value = 1000
exact = "10³"
tex = '10^3'

[area]
value = 600
exact = "6·10²"
tex = '6\cdot 10^2'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 0) to (10, 10, 10), its faces planes: the same cube. Unite them. Every face of one lies on a face of the other, and the union is that cube.

**As a set:**

$$[0,10]^3 \cup [0,10]^3$$
