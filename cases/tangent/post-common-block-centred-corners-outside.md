+++
title = "A post and a square block centred on it, corners outside the circle, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcommon_simple/ZA4"

[volume]
value = 621.897054604163923
exact = "V = 10·A, where A = 48 + 50·(arcsin(4/5) − arcsin(3/5)) is the area of the disc of radius 5 inside the block's outline"
tex = '10A,\quad A = 48 + 50\bigl(\arcsin\tfrac45 - \arcsin\tfrac35\bigr)'

[area]
value = 421.138232762498354
exact = "area = 2A + 10(a + ℓ), where a = 20·(arcsin(4/5) − arcsin(3/5)) is the length of the post's circle inside the block's outline and ℓ = 24 is the length of the outline's sides inside the disc"
tex = '2A + 10(a + \ell),\quad a = 20\bigl(\arcsin\tfrac45 - \arcsin\tfrac35\bigr),\ \ell = 24'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −4 to 4, y from −4 to 4 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Take the part common to the post and the block.

The block's outline is a square centred on the post's axis. Each of its four faces cuts across the post's circle, 4 from the axis, meeting the wall at (4, ±3), (−4, ±3), (±3, 4) and (±3, −4); its four corners (±4, ±4) are outside the post (32 > 25). No face is tangent to the post and no corner is on the wall.

What is left is the post with four flat sides cut into it, one by each face of the block, four wall patches remaining between them.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cap \bigl([-4,4]\times[-4,4]\times[0,10]\bigr)$$
