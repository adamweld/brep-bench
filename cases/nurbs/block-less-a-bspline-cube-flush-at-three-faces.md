+++
title = "A block less a B-spline cube, the block flush at three faces and overhanging"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/C1"

[volume]
value = 400
exact = "10 · 4 · (20 − 10)"
tex = '10\cdot 4\cdot(20-10)'

[area]
value = 360
exact = "2(10·4 + 10·10 + 4·10)"
tex = '2(10\cdot 4 + 10\cdot 10 + 4\cdot 10)'
+++

A block from (0, 0, 0) to (10, 4, 20), its faces planes. And a cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. Cut the cube from the block. The block's two ends and its sides at y = 0 and z = 0 lie in the cube's faces x = 0, x = 10, y = 0 and z = 0, and it runs on above the cube's top; its other long side, at y = 4, passes through the cube. What is left is the part above the cube, the box 10 by 4 by 10 from (0, 0, 10) to (10, 4, 20).

**As a set:**

$$([0,10]\times[0,4]\times[0,20]) \setminus [0,10]^3$$
