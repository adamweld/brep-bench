+++
title = "A post and a block tangent to it on four sides, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cylinder", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcommon_simple/R1"

[volume]
value = 785.3981633974482
exact = "the post itself: π·5²·10"
tex = '250\pi'

[area]
value = 471.23889803846896
exact = "wall 2π·5·10 plus two ends of 25π: 100π + 50π"
tex = '150\pi'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −5 to 5, y from −5 to 5 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Make the solid common to both.

The block's four sides x = ±5 and y = ±5 each touch the post along a line of its wall, and its corners (±5, ±5) are outside the post. The post is wholly inside the block, so the common part is the post.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cap \bigl([-5,5]^2\times[0,10]\bigr)$$
