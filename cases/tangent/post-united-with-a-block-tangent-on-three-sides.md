+++
title = "A post united with a block tangent on three sides, the fourth cutting across, flush at both ends"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "planar", "tangent-line", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/R3"

[volume]
value = 911.82380450040306
exact = "V = 10·|U|, |U| = 68 + 25θ, θ = arccos(3/5): the block's 80 plus the disc's 25π less the 25π − 25θ + 12 where they overlap"
tex = '10\,(68 + 25\theta),\quad \theta = \arccos\tfrac35'

[area]
value = 555.09428270024183
exact = "area = 2|U| + 10·P, P = 28 + 10θ (the block's outline outside the disc: 10 + 8 + 8 + 2, and the disc's arc beyond x = 3, 10θ)"
tex = '2\,(68 + 25\theta) + 10\,(28 + 10\theta),\quad \theta = \arccos\tfrac35'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −5 to 3, y from −5 to 5 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Make the union of the post and the block.

The faces x = −5 and y = ±5 each touch the post's wall along a line, where block and post are tangent. The face x = 3 cuts across the post, crossing its wall at y = ±4; the block's corners (3, ±5) are outside the post. The post's axis is inside the block's outline. The union is the block with the part of the post beyond x = 3 added to its side.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \bigl([-5,3]\times[-5,5]\times[0,10]\bigr)$$
