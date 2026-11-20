+++
title = "A B-spline cube grooved along a face"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/D9"

[volume]
value = 840.0
exact = "10³ − 10·4·4"
tex = '10^3 - 10\cdot 4\cdot 4'

[area]
value = 648.0
exact = "2·(10² − 4·4) + 10·(40 − 4 + 12)"
tex = '2\,(10^2 - 4\cdot 4) + 10\,(40 - 4 + 12)'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 3) to (10, 4, 7), its faces planes. Cut the block from the cube. The block's two ends lie in the cube's end faces x = 0 and x = 10 and its side at y = 0 in the cube's face y = 0; its other three sides are inside the cube, at y = 4, z = 3 and z = 7. A groove 4 deep and 4 wide is left across the whole cube, open on the face y = 0 and on both ends.

**As a set:**

$$[0,10]^3 \setminus ([0,10]\times[0,4]\times[3,7])$$
