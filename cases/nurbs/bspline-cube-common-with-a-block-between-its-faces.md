+++
title = "A B-spline cube and a block set between its top and bottom, their common part"
tags = ["nurbs", "bspline", "boolean", "common", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcommon_simple/B5"

[volume]
value = 300
exact = "10·10·3"
tex = '10\cdot 10\cdot 3'

[area]
value = 320
exact = "2·100 + 4·10·3"
tex = '2\cdot 100 + 4\cdot 10\cdot 3'
+++

A B-spline cube and a block set between its top and bottom, their common part

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 3) to (10, 10, 6), its faces planes. Make the solid common to both.

The block's four sides lie in the cube's, and its top and bottom lie inside the cube, clear of the cube's. The common part is the block.

**As a set:**

$$[0,10]^3 \cap [0,10]\times[0,10]\times[3,6]$$
