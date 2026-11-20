+++
title = "A post and a block tangent to it twice with a corner inside, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcommon_simple/T3"

[volume]
value = 305.678815902608337
exact = "V = 10·A, where A = 25π/4 + 25·arcsin(1/5) + 1 + 2·√6 is the area of the disc of radius 5 inside the block's outline"
tex = '10A,\quad A = \tfrac{25\pi}{4} + 25\arcsin\tfrac15 + 1 + 2\sqrt6'

[area]
value = 277.790961310626701
exact = "area = 2A + 10(a + ℓ), where a = 5·(π/2 + 2·arcsin(1/5)) is the length of the post's circle inside the block's outline and ℓ = 2 + 4·√6 is the length of the outline's sides inside the disc"
tex = '2A + 10(a + \ell),\quad a = 5\bigl(\tfrac{\pi}{2} + 2\arcsin\tfrac15\bigr),\ \ell = 2 + 4\sqrt6'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −1 to 5, y from −1 to 5 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Take the part common to the post and the block.

The block's faces x = 5 and y = 5 are each tangent to the post's wall, along the lines (5, 0) and (0, 5); each tangent line lies inside its face. The corner (−1, −1) is inside the post; the corners (5, −1) and (−1, 5) are just outside it (26 > 25) and (5, 5) is far outside. The post's axis is inside the block's outline.

What is left is the part of the post with x ≥ −1 and y ≥ −1: the post with a corner trimmed off by the two planes x = −1 and y = −1.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cap \bigl([-1,5]\times[-1,5]\times[0,10]\bigr)$$
