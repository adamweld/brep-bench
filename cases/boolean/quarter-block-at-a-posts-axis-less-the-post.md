+++
title = "A block with a corner on a post's axis, covering a quarter of it, less the post"
tags = ["boolean", "cut", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/boptuc_simple/ZA8"

[volume]
value = 163.65045915063792
exact = "V = 10 (36 − 25π/4): the block's section less the quarter of the post's disc inside it"
tex = '10\,\bigl(36 - \tfrac{25\pi}{4}\bigr)'

[area]
value = 251.26990816987242
exact = "area = 2 (36 − 25π/4) + 10 (14 + 5π/2): the block's outline outside the post is 24 less its faces x = 0 and y = 0 from the axis to the wall, 5 each, and the wall inside the block is a quarter of 10π"
tex = '2\,\bigl(36 - \tfrac{25\pi}{4}\bigr) + 10\,\bigl(14 + \tfrac{5\pi}{2}\bigr)'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from 0 to 6, y from 0 to 6 and z from 0 to 10: a square 6 on a side with one corner on the post's axis, covering a quarter of the post, and as tall as the post, its top and bottom faces level with the post's. Cut the post away from the block.

The block's faces x = 0 and y = 0 run through the post's axis and meet its wall at (0, 5) and (5, 0). Its faces x = 6 and y = 6 and its corners (6, 0), (0, 6) and (6, 6) are all outside the post. What is left is the block outside the quarter-round of the post: one piece, thickest at the corner (6, 6) and thinning to 1 along the two faces through the axis.

**As a set:**

$$\bigl([0,6]^2\times[0,10]\bigr) \setminus \{\,\rho \le 5,\ 0 \le z \le 10\,\}$$
