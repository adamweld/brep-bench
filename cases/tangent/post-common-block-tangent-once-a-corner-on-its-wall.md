+++
title = "A post and a block, one face tangent to it at the block's corner, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcommon_simple/U4"

[volume]
value = 336.787179448522626
exact = "V = 10·A, where A = 25π/4 + 6 + (25/2)·arcsin(3/5) is the area of the disc of radius 5 inside the block's outline"
tex = '10A,\quad A = \tfrac{25\pi}{4} + 6 + \tfrac{25}{2}\arcsin\tfrac35'

[area]
value = 298.072307669113575
exact = "area = 2A + 10(a + ℓ), where a = 5·(π/2 + arcsin(3/5)) is the length of the post's circle inside the block's outline and ℓ = 12 is the length of the outline's sides inside the disc"
tex = '2A + 10(a + \ell),\quad a = 5\bigl(\tfrac{\pi}{2} + \arcsin\tfrac35\bigr),\ \ell = 12'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from 0 to 8, y from −3 to 5 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Take the part common to the post and the block.

The block's face y = 5 is tangent to the post's wall along the line x = 0, y = 5, and that line is also a vertical edge of the block: the block's corner (0, 5) lies exactly on the wall. The face x = 0 passes through the post's axis, so the axis lies on the block's boundary. The face y = −3 cuts across the post's circle, meeting it at x = 4; the face x = 8 is clear of the post. Of the other corners, (0, −3) is inside the post and (8, 5) and (8, −3) are outside.

What is left is the part of the post with x ≥ 0 and y ≥ −3: its flat faces are x = 0 and y = −3, and the post's wall is cut along the line x = 0, y = 5, where the block's tangent plane touches it.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cap \bigl([0,8]\times[-3,5]\times[0,10]\bigr)$$
