+++
title = "A B-spline cube united with a block resting on its top and overhanging an edge"
tags = ["nurbs", "bspline", "boolean", "union", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bfuse_simple/C2, OCCT boolean/bfuse_simple/C7"

[volume]
value = 1500
exact = "10³ + 10·10·5"
tex = '10^3 + 10\cdot 10\cdot 5'

[area]
value = 900
exact = "600 + 400 − 2·(10·5)"
tex = '600 + 400 - 2\,(10\cdot 5)'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, −5, 10) to (10, 5, 15), its faces planes. Unite them.

The block rests on the cube's top: its bottom face lies in the plane of the cube's top, and the two touch over the strip 0 ≤ y ≤ 5 of it, the half of the top next to the y = 0 edge. The other half of the block's bottom, from y = −5 to y = 0, hangs out past the cube's y = 0 side with nothing under it. No volume overlaps. The block's sides at x = 0 and x = 10 lie in the planes of the cube's sides at x = 0 and x = 10. The union is one solid, the cube with a block 10 by 10 by 5 on top of it, shifted half its depth out past one side.

**As a set:**

$$[0,10]^3 \cup [0,10]\times[-5,5]\times[10,15]$$
