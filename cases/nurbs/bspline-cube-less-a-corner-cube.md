+++
title = "A B-spline cube less a smaller block at its corner"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/C6"

[volume]
value = 936
exact = "10³ − 4³"
tex = '10^3 - 4^3'

[area]
value = 600
exact = "6·10² − 3·4² + 3·4²"
tex = '6\cdot 10^2 - 3\cdot 4^2 + 3\cdot 4^2'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 0) to (4, 4, 4), its faces planes. Cut the block from the cube. The block, a cube of side 4, shares the cube's corner at the origin and lies against its three faces there. A square notch is left at that corner.

**As a set:**

$$[0,10]^3 \setminus ([0,4]\times[0,4]\times[0,4])$$
