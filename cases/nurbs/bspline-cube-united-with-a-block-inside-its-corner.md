+++
title = "A B-spline cube united with a block inside it, flush at three of its faces"
tags = ["nurbs", "bspline", "boolean", "union", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bfuse_simple/B1, OCCT boolean/bfuse_simple/B5, OCCT boolean/bfuse_simple/B6"

[volume]
value = 1000
exact = "10³"
tex = '10^3'

[area]
value = 600
exact = "6·10²"
tex = '6\cdot 10^2'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 0) to (5, 5, 5), its faces planes: it fills one corner of the cube, three of its faces lying in three of the cube's faces and its other three inside the cube. Unite them. The block adds nothing, and the union is the cube.

**As a set:**

$$[0,10]^3 \cup [0,5]^3$$
