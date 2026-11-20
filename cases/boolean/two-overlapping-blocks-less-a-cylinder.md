+++
title = "Two overlapping blocks, united, less a cylinder along an edge of one"
tags = ["boolean", "cut", "analytic", "planar", "cylinder", "coincident"]
origin = "FreeCAD src/Mod/Part/parttests/TopoShapeTest.py::testPartCompoundCut2"

[volume]
value = 1499.5799175824626
exact = "10·(252 − 36π + 6π − 9√3/2) = 2520 − 300π − 45√3: the union's section (100 + 176 − 24 = 252) less its part inside the disc (the disc less the leftover segment)"
tex = '2520 - 300\pi - 45\sqrt3'

[area]
value = 1252.113743074868
exact = "2(252 − 30π − 9√3/2) + 10(69 − 3√3 + 10π) = 1194 + 40π − 39√3: the two ends, and the section's boundary run 10: the union's outline of 72, less 3 and 3√3 inside the disc, plus the wall's arc of 10π"
tex = '2\left(252 - 30\pi - \tfrac{9\sqrt3}{2}\right) + 10\left(69 - 3\sqrt3 + 10\pi\right) = 1194 + 40\pi - 39\sqrt3'
+++

Two blocks, both from z = 0 to z = 10: the first from (0, 0, 0) to (10, 10, 10), the second from (−8, −8, 0) to (8, 3, 10). They overlap in the block from (0, 0, 0) to (8, 3, 10). Unite them, then cut away the cylinder of radius 6 about the z axis, from z = 0 to z = 10; the axis is the first block's vertical edge at x = y = 0. The cylinder's ends lie in the planes that the blocks' tops and bottoms share.

The cylinder takes the first block's corner and the second's middle. Its wall meets the united blocks' outline at (0, 6) on the plane x = 0 and at (−3√3, 3) on the plane y = 3; the only part of the cylinder outside the blocks is the piece with x < 0 and y > 3.

**As a set:**

$$\Bigl(\bigl([0,10]^2\times[0,10]\bigr) \cup \bigl([-8,8]\times[-8,3]\times[0,10]\bigr)\Bigr) \setminus \bigl(\{\,x^2 + y^2 \le 6^2\,\}\times[0,10]\bigr)$$
