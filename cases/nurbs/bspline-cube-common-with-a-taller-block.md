+++
title = "A B-spline cube and a block twice as high, their common part"
tags = ["nurbs", "bspline", "boolean", "common", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcommon_simple/A6, OCCT boolean/bcommon_simple/A7"

[volume]
value = 1000
exact = "10³"
tex = '10^3'

[area]
value = 600
exact = "6·10²"
tex = '6\cdot 10^2'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 0) to (10, 10, 20), its faces planes. Make the solid common to both.

The block's four sides and its bottom lie in the cube's, and the block's top is 10 above the cube's, so the cube lies wholly in the block and the common part is the cube. Its faces are the cube's B-spline faces, each lying on a face of the block.

**As a set:**

$$[0,10]^3 \cap [0,10]\times[0,10]\times[0,20]$$
