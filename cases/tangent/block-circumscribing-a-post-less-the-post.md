+++
title = "A block circumscribing a post, less the post"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar", "tangent-line"]
origin = "OCCT tests/boolean/boptuc_simple/R1"

[volume]
value = 214.60183660255169
exact = "V = 10 (100 − 25π): the block's section less the post's disc"
tex = '10\,(100 - 25\pi)'

[area]
value = 757.07963267948966
exact = "area = 2·(100 − 25π) (top and bottom) + 10·40 (the block's four sides, each touching the post along a line and otherwise whole) + 10·10π (the post's wall, all of it inside the block's outline): 600 + 50π"
tex = '2\,(100 - 25\pi) + 400 + 100\pi = 600 + 50\pi'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −5 to 5, y from −5 to 5 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Cut the post away from the block.

The block's four side faces, x = ±5 and y = ±5, each touch the post's wall along a line, where block and post are tangent: the post stands inside the block's outline, filling it from side to side. The block's corners are outside the post.

What is left is the block's four corners, each between two of its sides and the post's wall. The part left can be in several pieces, even pieces that touch only along a line; build it all as one result, and measure it whole.

**As a set:**

$$\bigl([-5,5]\times[-5,5]\times[0,10]\bigr) \setminus \{\,\rho \le 5,\ 0 \le z \le 10\,\}$$
