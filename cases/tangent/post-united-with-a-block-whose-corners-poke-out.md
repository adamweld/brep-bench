+++
title = "A post united with a square block whose sides are inside the post and whose corners poke out, flush at both ends"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "planar", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/ZA5"

[volume]
value = 803.50110879328439
exact = "V = 10·|U|, |U| = 16 + 100φ, φ = arccos(4/5): the block's 64 plus the disc's 25π less the 25π − 100φ + 48 where they overlap"
tex = '10\,(16 + 100\varphi),\quad \varphi = \arccos\tfrac45'

[area]
value = 498.10066527597063
exact = "area = 2|U| + 10·P, P = 8 + 40φ (the block's outline outside the disc: four sides, each with two pieces of length 1, and the disc's four arcs between the sides, 10φ each)"
tex = '2\,(16 + 100\varphi) + 10\,(8 + 40\varphi),\quad \varphi = \arccos\tfrac45'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block of square section, x and y from −4 to 4, and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Make the union of the post and the block.

The block's side faces, x = ±4 and y = ±4, are at distance 4 from the axis and cross the post's wall only near their middles, where |y| < 3 (or |x| < 3); its corners, at distance 4√2 ≈ 5.66 from the axis, are outside the post. Four small corner pieces of the block stand out of the post's wall.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \bigl([-4,4]^2\times[0,10]\bigr)$$
