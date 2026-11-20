+++
title = "A quarter of a cylinder, its ends moved out"
tags = ["edit", "move-face", "cylinder", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/I3, I4, I7"

[volume]
value = 863.9379797371931
exact = "(πR²/4)·h = (π·100/4)·11"
tex = '\tfrac{\pi R^2}{4}\, h = \tfrac{\pi\cdot 100}{4}\cdot 11'

[area]
value = 549.8672286269283
exact = "2·(π·100/4) + 11·(5π + 20) = 105π + 220: the two ends, the wall (arc 5π) and the two flats (10 wide)"
tex = '2\cdot\tfrac{100\pi}{4} + 11\left(5\pi + 20\right) = 105\pi + 220'
+++

A quarter of a cylinder: the part of the cylinder of radius 10 about the z axis, from z = 0 to z = 6, whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 90°, so the part with x ≥ 0 and y ≥ 0. Its faces are the wall, a quarter of a cylinder; a bottom and a top, quarter disks; and two flat rectangles, 10 by 6, in the half-planes y = 0, x ≥ 0 and x = 0, y ≥ 0, which meet along the z axis in a convex edge.

Then edit the solid directly: move the bottom down by 2 and the top up by 3, each along its outward normal, to the planes z = −2 and z = 9. The wall and the two flats run on to meet them.

So the result is the same quarter of a cylinder, from z = −2 to z = 9.

**As a set:**

$$\{\, x^2 + y^2 \le 10^2,\ x \ge 0,\ y \ge 0,\ -2 \le z \le 9 \,\}$$
