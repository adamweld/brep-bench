+++
title = "A post and a block in one quadrant with three corners inside it, their common part"
tags = ["boolean", "common", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcommon_simple/ZC3"

[volume]
value = 77.73713182552049
exact = "V = 5A, A = 12 + (25/2)(arcsin(4/5) − arcsin(3/5)) is the area of the disc of radius 5 inside the square [0, 4]²"
tex = '5A,\quad A = 12 + \tfrac{25}{2}\bigl(\arcsin\tfrac45 - \arcsin\tfrac35\bigr)'

[area]
value = 108.18970546041639
exact = "area = 2A + 5P, P = 14 + 5(arcsin(4/5) − arcsin(3/5)) is the length of the outline of that region"
tex = '2A + 5P,\quad P = 14 + 5\bigl(\arcsin\tfrac45 - \arcsin\tfrac35\bigr)'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from 0 to 4, y from 0 to 4 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base, its top face at z = 5 inside the post. Take the part common to the post and the block.

The block's corner (0, 0) is on the post's axis, so the axis lies on the block's boundary; its corners (4, 0) and (0, 4) are inside the post and (4, 4) is outside it (32 > 25). The faces x = 4 and y = 4 cut across the post's circle, meeting the wall at (4, 3) and (3, 4); the faces x = 0 and y = 0 pass through the axis.

What is left is the block with its corner at (4, 4) trimmed off by the post's wall: a patch of wall stands in place of that corner.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cap \bigl([0,4]\times[0,4]\times[0,5]\bigr)$$
