+++
title = "A block less a tall cylinder grazing its side edges"
tags = ["boolean", "cut", "analytic", "tangent", "coincident", "planar", "pieces"]
origin = "OCCT tests/boolean/bopcut_simple/ZC7"

[volume]
value = 9.975011212410129
exact = "two slivers, each of area S = the integral from 0 to 3 of (4 − √(16 − x²)) dx = 12 − 3√7/2 − 8·asin(3/4), times the height 4"
tex = '8\left(12 - \tfrac{3\sqrt7}{2} - 8\arcsin\tfrac34\right)'

[area]
value = 66.95948164509574
exact = "top and bottom, 2·(2S); the face x = 3 keeps 4 − √7 of its width on each side of the cylinder, 2(4 − √7)·4; the faces y = ±4 are whole, 2·(3·4); the cylinder's wall inside the block is two arcs of 4·asin(3/4), times the height 4; the face x = 0 lies inside the cylinder and goes"
tex = '4S + 8(4-\sqrt7) + 24 + 32\arcsin\tfrac34,\quad S = 12 - \tfrac{3\sqrt7}{2} - 8\arcsin\tfrac34'
+++

A block from (0, −4, 2) to (3, 4, 6), and a cylinder of radius 4 standing on the xy plane with its axis the z axis, from z = 0 to z = 8, so that it is taller than the block and runs past both its faces z = 2 and z = 6. Cut the cylinder away from the block.

The block's width, 8, is the cylinder's diameter. Its face x = 0 passes through the axis, its edges at (0, ±4) lie on the cylinder's wall, and its faces y = ±4 are tangent to the cylinder along those edges. What is left is two separate slivers of the block, one in y ≥ 0 and one in y ≤ 0, each a corner of the block cut off by the round wall and narrowing to a cusp at its edge on x = 0.

**As a set:**

$$\bigl([0,3]\times[-4,4]\times[2,6]\bigr) \setminus \{\rho \le 4,\ 0 \le z \le 8\}$$
