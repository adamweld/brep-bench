+++
title = "A B-spline cube and a block at its corner, flush at three faces, their common part"
tags = ["nurbs", "bspline", "boolean", "common", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcommon_simple/B6"

[volume]
value = 120
exact = "6·5·4"
tex = '6\cdot 5\cdot 4'

[area]
value = 148
exact = "2·(30 + 24 + 20)"
tex = '2\,(30 + 24 + 20)'
+++

A B-spline cube and a block at its corner, flush at three faces, their common part

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 0) to (6, 5, 4), its faces planes. Make the solid common to both.

The block lies wholly in the cube, and its faces at x = 0, y = 0 and z = 0 lie in the cube's. The common part is the block.

**As a set:**

$$[0,10]^3 \cap [0,6]\times[0,5]\times[0,4]$$
