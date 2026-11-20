+++
title = "A post and a block with a face through its axis, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcommon_simple/ZA6"

[volume]
value = 194.530725415988805
exact = "V = 10·A, where A = 2·√21 + 25·arcsin(2/5) is the area of the disc of radius 5 inside the block's outline"
tex = '10A,\quad A = 2\sqrt{21} + 25\arcsin\tfrac25'

[area]
value = 211.709343589063363
exact = "area = 2A + 10(a + ℓ), where a = 10·arcsin(2/5) is the length of the post's circle inside the block's outline and ℓ = 4 + 2·√21 is the length of the outline's sides inside the disc"
tex = '2A + 10(a + \ell),\quad a = 10\arcsin\tfrac25,\ \ell = 4 + 2\sqrt{21}'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from 0 to 6, y from −2 to 2 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Take the part common to the post and the block.

The block's face x = 0 contains the post's axis, so the axis lies on the block's boundary, not inside it. Its corners (0, ±2) are inside the post and (6, ±2) are outside; no face is tangent to the post and no corner is on the wall.

What is left is the part of the post with x ≥ 0 and −2 ≤ y ≤ 2: a slab with three flat faces (x = 0, y = −2, y = 2) and a patch of the post's wall, with the post's flat top and bottom.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cap \bigl([0,6]\times[-2,2]\times[0,10]\bigr)$$
