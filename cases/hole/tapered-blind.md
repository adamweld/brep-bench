+++
title = "A tapered blind hole, wider at its mouth"
tags = ["hole", "tapered", "blind", "cut", "planar", "cone", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestHole.py::testTaperedHole"

[volume]
value = 46094.100456822192
exact = "40·40·30 − (π·20/3)(6² + 6·5 + 5²) = 48000 − 1820π/3: the block less the frustum the hole is"
tex = '40\cdot 40\cdot 30 - \tfrac{\pi\cdot 20}{3}\left(6^2 + 6\cdot 5 + 5^2\right) = 48000 - \tfrac{1820\pi}{3}'

[area]
value = 8657.4562632926654
exact = "8000 − π·6² + π·5² + π(6 + 5)√(20² + 1²) = 8000 − 11π + 11π√401: the block, less the mouth, and the hole's floor and its conical wall"
tex = '8000 - \pi\cdot 6^2 + \pi\cdot 5^2 + \pi(6 + 5)\sqrt{20^2 + 1^2} = 8000 - 11\pi + 11\pi\sqrt{401}'
+++

A block, the box from the origin to (40, 40, 30), with a tapered blind hole drilled down into it
from the top face, its axis the vertical line through (20, 20).

The hole is round and narrows as it goes down: radius 6 at its mouth in the top face (z = 30) and
radius 5 at its floor, 20 down at z = 10, its radius falling evenly with depth in between, so its
wall is a cone about the axis, leaning 1 in 20 from it. The floor is flat, the disk of radius 5 at
z = 10. The hole does not reach the bottom face.

**As a set:**

$$\bigl([0,40]^2\times[0,30]\bigr) \setminus \{\, d \le 5 + \tfrac{z - 10}{20},\ z \ge 10 \,\},\quad d = \sqrt{(x-20)^2 + (y-20)^2}$$
