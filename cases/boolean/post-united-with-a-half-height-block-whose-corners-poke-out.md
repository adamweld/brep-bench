+++
title = "A post united with a square block half as tall, its sides inside the post and its corners poking out"
tags = ["boolean", "union", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopfuse_simple/ZB7"

[volume]
value = 794.4496360953664
exact = "V = 250π + 5·(100φ − 25π + 16), φ = arccos(4/5): the post, and the block's part outside the post's disc (the block's 64 less the 25π − 100φ + 48 inside the disc) up to z = 5"
tex = '250\pi + 5\,(100\varphi - 25\pi + 16),\quad \varphi = \arccos\tfrac45'

[area]
value = 486.4800761968034
exact = "area = (16 + 100φ) (base: the disc and the corners) + 25π (post's top) + (100φ − 25π + 16) (block's top, the four corners) + 8·5 (block's sides, each with two pieces of length 1) + 40φ·5 (post's wall below z = 5, four arcs of 10φ) + 2π·5·5 (post's wall above z = 5)"
tex = '(16 + 100\varphi) + 25\pi + (100\varphi - 25\pi + 16) + 40 + 200\varphi + 50\pi,\quad \varphi = \arccos\tfrac45'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block of square section, x and y from −4 to 4, and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base. Make the union of the post and the block.

The block's side faces, x = ±4 and y = ±4, are at distance 4 from the axis and cross the post's wall only near their middles, where |y| < 3 (or |x| < 3); its corners, at distance 4√2 ≈ 5.66 from the axis, are outside the post. Four small corner pieces of the block stand out of the post's wall, up to z = 5, and above z = 5 the post stands whole.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \bigl([-4,4]^2\times[0,5]\bigr)$$
