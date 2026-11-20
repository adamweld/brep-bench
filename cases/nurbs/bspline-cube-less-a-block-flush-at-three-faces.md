+++
title = "A B-spline cube less a block that overhangs it, flush at three faces"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/B9"

[volume]
value = 600
exact = "10·(10 − 4)·10"
tex = '10\cdot(10-4)\cdot 10'

[area]
value = 440
exact = "2(6·10 + 10·10 + 6·10)"
tex = '2(6\cdot 10 + 10\cdot 10 + 6\cdot 10)'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a block from (0, 0, 0) to (10, 4, 20), its faces planes. Cut the block from the cube. The block's two ends and its sides at y = 0 and z = 0 lie in the cube's faces x = 0, x = 10, y = 0 and z = 0, and it runs on above the cube's top; its other long side, at y = 4, passes through the cube. It takes a slab 4 thick off the cube's side at y = 0, and the box 10 by 6 by 10 from (0, 4, 0) to (10, 10, 10) is left.

**As a set:**

$$[0,10]^3 \setminus ([0,10]\times[0,4]\times[0,20])$$
