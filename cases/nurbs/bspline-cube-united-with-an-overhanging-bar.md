+++
title = "A B-spline cube united with a bar that rises past its top"
tags = ["nurbs", "bspline", "boolean", "union", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bfuse_simple/B3, OCCT boolean/bfuse_simple/C1"

[volume]
value = 1500
exact = "1000 + 10·5·10"
tex = '1000 + 10\cdot 5\cdot 10'

[area]
value = 900
exact = "2·150 + 60·10: the L-shaped profile in the yz plane has area 10·10 + 5·10 = 150 and perimeter 60, and the solid is that profile extruded 10 along x"
tex = '2\cdot 150 + 60\cdot 10'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 0) to (10, 5, 20), its faces planes. Unite them. The block spans the cube's whole width in x, covers half its depth in y starting at the y = 0 face, and runs from the cube's bottom to twice its height: its sides at x = 0, x = 10 and y = 0 and its bottom lie in the cube's faces, and the half above the cube's top rises from it. The union is the cube with a bar 10 by 5 by 10 standing on its top along the y = 0 edge, an L in profile.

**As a set:**

$$[0,10]^3 \cup [0,10]\times[0,5]\times[0,20]$$
