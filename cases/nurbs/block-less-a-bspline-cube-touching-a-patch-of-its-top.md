+++
title = "A block less a B-spline cube touching a patch of its top"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/B8, OCCT boolean/bcut_simple/C9"

[volume]
value = 160
exact = "4 · 4 · 10"
tex = '4\cdot 4\cdot 10'

[area]
value = 192
exact = "2·4² + 4·4·10"
tex = '2\cdot 4^2 + 4\cdot(4\cdot 10)'
+++

A block from (0, 0, 10) to (4, 4, 20), its faces planes. And a cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. Cut the cube from the block. The block stands on the cube's top face over a 4 by 4 patch at its corner at the origin, sharing only that patch, with no volume in common. Nothing is removed: the result is the whole block.

**As a set:**

$$([0,4]\times[0,4]\times[10,20]) \setminus [0,10]^3$$
