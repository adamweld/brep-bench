+++
title = "A post united with a bar tangent on two opposite sides, flush at both ends"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "planar", "tangent-line", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/S9"

[volume]
value = 823.64760900080612
exact = "V = 10·|U|, |U| = 36 + 50θ, θ = arccos(3/5): the block's 60 plus the disc's 25π less the 25π − 50θ + 24 where they overlap"
tex = '10\,(36 + 50\theta),\quad \theta = \arccos\tfrac35'

[area]
value = 510.18856540048367
exact = "area = 2|U| + 10·P, P = 16 + 20θ (the block's outline outside the disc: 6 + 6 + 4·1, and the disc's two arcs beyond y = ±3, 10θ each)"
tex = '2\,(36 + 50\theta) + 10\,(16 + 20\theta),\quad \theta = \arccos\tfrac35'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −5 to 5, y from −3 to 3 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Make the union of the post and the block.

The faces x = ±5 each touch the post's wall along a line, where block and post are tangent. The faces y = ±3 cut across the post, crossing its wall at x = ±4. The block's corners (±5, ±3) are outside the post. The union is the post with four corner pieces of the block added.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \bigl([-5,5]\times[-3,3]\times[0,10]\bigr)$$
