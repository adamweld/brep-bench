+++
title = "A post united with a block circumscribing it, flush at both ends"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "planar", "tangent-line", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/R1"

[volume]
value = 1000.0
exact = "V = 10 · 10 · 10 (the post lies inside the block)"
tex = '10\cdot 10\cdot 10'

[area]
value = 600.0
exact = "area = 6 · 10² (the union is the block)"
tex = '6\cdot 10^2'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x and y from −5 to 5 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Make the union of the post and the block.

The block's four side faces, x = ±5 and y = ±5, each touch the post's wall along a line, where block and post are tangent. The post stands inside the block's outline, and the union is the block itself.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \bigl([-5,5]^2\times[0,10]\bigr)$$
