+++
title = "A cylinder less two overlapping blocks, its axis along an edge of one"
tags = ["boolean", "cut", "analytic", "planar", "cylinder", "coincident"]
origin = "FreeCAD src/Mod/Part/parttests/TopoShapeTest.py::testPartCompoundCut1"

[volume]
value = 110.55327287478812
exact = "10·(6π − 9√3/2): the segment of the disc with y ≥ 3, its half with x ≤ 0, 10 high"
tex = '10\left(6\pi - \tfrac{9\sqrt3}{2}\right)'

[area]
value = 166.90403187381981
exact = "2(6π − 9√3/2) + 30√3 + 30 + 20π: the two ends, the cut at y = 3 (3√3 × 10), the cut at x = 0 (3 × 10), the wall (an arc of π/3 at radius 6, × 10)"
tex = '2\left(6\pi - \tfrac{9\sqrt3}{2}\right) + 30\sqrt3 + 30 + 20\pi = 32\pi + 30 + 21\sqrt3'
+++

A cylinder of radius 6 about the z axis, from z = 0 to z = 10. Two blocks, both from z = 0 to z = 10: the first from (0, 0, 0) to (10, 10, 10), whose vertical edge at x = y = 0 is the cylinder's axis, and the second from (−8, −8, 0) to (8, 3, 10). The blocks overlap each other in the block from (0, 0, 0) to (8, 3, 10). Cut both blocks away from the cylinder. The cylinder's ends lie in the planes that the blocks' tops and bottoms share.

What is left is the part of the cylinder with x < 0 and y > 3: one piece, bounded by the two ends, the plane y = 3 (a face of the second block), the plane x = 0 (a face of the first) and the cylinder's wall.

**As a set:**

$$\{\,x^2 + y^2 \le 6^2\,\}\times[0,10] \setminus \Bigl(\bigl([0,10]^2\times[0,10]\bigr) \cup \bigl([-8,8]\times[-8,3]\times[0,10]\bigr)\Bigr)$$
