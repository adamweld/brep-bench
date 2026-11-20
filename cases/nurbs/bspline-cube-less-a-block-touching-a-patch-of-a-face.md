+++
title = "A B-spline cube less a block touching a patch of its top"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/B7, OCCT boolean/bcut_simple/C8"

[volume]
value = 1000
exact = "10³"
tex = '10^3'

[area]
value = 600
exact = "6 · 10²"
tex = '6\cdot 10^2'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 10) to (4, 4, 20), its faces planes. Cut the block from the cube. The block stands on the cube's top face over a 4 by 4 patch at its corner at the origin, sharing only that patch, with no volume in common. Nothing is removed: the result is the whole cube.

**As a set:**

$$[0,10]^3 \setminus ([0,4]\times[0,4]\times[10,20])$$
