+++
title = "A block less a drum tangent to two of its faces"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar", "tangent-line"]
origin = "OCCT tests/boolean/boptuc_simple/ZC6"

[volume]
value = 82.92036732051034
exact = "height 4 times the block's footprint less the half disc of the drum in it: 4(10·6 − πr²/2), r = 5"
tex = '4\left(10\cdot 6 - \tfrac{\pi r^2}{2}\right),\quad r = 5'

[area]
value = 192.29203673205103
exact = "top and bottom 2(60 − 25π/2); the faces x = ±5, 6·4 each, 48; the face y = 6, 10·4 = 40; the face y = 0 lies inside the drum and is gone; the drum's wall, a half circle of length 5π by height 4, 20π"
tex = '2\left(60 - \tfrac{25\pi}{2}\right) + 48 + 40 + 20\pi = 208 - 5\pi'
+++

A drum: a cylinder of radius 5 with the z axis as its axis, from z = 0 to z = 10. A block with x from −5 to 5, y from 0 to 6 and z from 3 to 7, at mid-height of the drum, touching neither end. The block covers the half of the drum's section with y ≥ 0 and runs on beyond it to y = 6. Its faces x = −5 and x = 5 are tangent to the drum, each touching its wall along a vertical line, at (−5, 0) and (5, 0), where they meet the block's face y = 0. Cut the drum away from the block.

What is left is the part of the block outside the drum, in one piece: a bar along y = 6 whose two ends run down the sides and thin to a cusp at (±5, 0), 4 high, where the face x = ±5 and the drum's wall meet tangentially. Its inner wall is the half of the drum's wall with y ≥ 0, concave, 4 high.

**As a set:**

$$\bigl([-5,5]\times[0,6]\times[3,7]\bigr) \setminus \{\,\rho \le 5,\ 0 \le z \le 10\,\}$$
