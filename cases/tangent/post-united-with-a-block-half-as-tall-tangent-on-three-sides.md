+++
title = "A post united with a block half as tall, tangent on three sides, flush at the base"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "planar", "tangent-line", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/V7"

[volume]
value = 848.61098394892568
exact = "V = 250π + 5·(68 − 25π + 25θ), θ = arccos(3/5): the post, and the block's part outside the disc up to z = 5"
tex = '250\pi + 5\,(68 - 25\pi + 25\theta),\quad \theta = \arccos\tfrac35'

[area]
value = 525.80915447965089
exact = "area = (68 + 25θ) (base) + 25π (post's top) + (68 − 25π + 25θ) (block's top outside the disc) + 5·28 (block's sides outside the disc) + 10·10θ (wall beyond x = 3, full height) + 5·(10π − 10θ) (the rest of the wall, z from 5 to 10)"
tex = '(68 + 25\theta) + 25\pi + (68 - 25\pi + 25\theta) + 140 + 100\theta + 5\,(10\pi - 10\theta)'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −5 to 3, y from −5 to 5 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base. Make the union of the post and the block.

The faces x = −5 and y = ±5 each touch the post's wall along a line, where block and post are tangent, from z = 0 to z = 5. The face x = 3 cuts across the post, crossing its wall at y = ±4; the block's corners (3, ±5) are outside the post. Above z = 5 the post stands whole.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \bigl([-5,3]\times[-5,5]\times[0,5]\bigr)$$
