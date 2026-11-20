+++
title = "A post united with a block half as tall circumscribing it, flush at the base"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "planar", "tangent-line", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/V5"

[volume]
value = 892.69908169872415
exact = "V = 250π + 5·(100 − 25π) (the post, and the block's corners beside it up to z = 5)"
tex = '250\pi + 5\,(100 - 25\pi)'

[area]
value = 557.07963267948966
exact = "area = 100 (base) + 25π (post's top) + (100 − 25π) (block's top, the corners) + 4·10·5 (block's sides) + 2π·5·5 (post's wall above z = 5)"
tex = '100 + 25\pi + (100 - 25\pi) + 200 + 50\pi'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x and y from −5 to 5 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base. Make the union of the post and the block.

The block's four side faces, x = ±5 and y = ±5, each touch the post's wall along a line where block and post are tangent, from z = 0 to z = 5. Above z = 5 the post stands whole. The block's top face is the square of side 10 less the post's disc: four corner pieces, which meet the post's wall only at the four points where the tangent lines end.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \bigl([-5,5]^2\times[0,5]\bigr)$$
