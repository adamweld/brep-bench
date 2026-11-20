+++
title = "A B-spline cube united with a bar along its diagonal"
tags = ["nurbs", "bspline", "boolean", "union", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bfuse_simple/D1, OCCT boolean/bfuse_simple/C8"

[volume]
value = 1040
exact = "10³ + 400 − 360"
tex = '10^3 + 10\sqrt2\cdot 2\sqrt2\cdot 10 - 360'

[area]
value = 641.1370849898476
exact = "2·(100 + 4) + 320 + 80√2"
tex = '2\,(100 + 4) + 320 + 80\sqrt2'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a bar from z = 0 to z = 10 whose footprint is the rectangle with corners (1, −1), (11, 9), (9, 11) and (−1, 1), its faces planes: it is 10√2 long, as long as the cube's diagonal, and 2√2 wide, and its axis runs along the diagonal of the cube's bottom from the corner (0, 0) to the corner (10, 10), its two ends square to the diagonal and centred on those corners. Unite them.

The bar's long sides are the planes y = x + 2 and y = x − 2, and its ends the planes x + y = 0 and x + y = 20. The bar lies inside the cube except for a small triangle of its footprint at each end, which stands out past the cube's two sides at that corner: the triangles (0, 0), (−1, 1), (0, 2) and (0, 0), (1, −1), (2, 0) at the first end, and their counterparts at the corner (10, 10). The two solids share the planes z = 0 and z = 10.

**As a set:**

$$[0,10]^3 \cup \{\,|x - y| \le 2,\ 0 \le x + y \le 20,\ 0 \le z \le 10\,\}$$
