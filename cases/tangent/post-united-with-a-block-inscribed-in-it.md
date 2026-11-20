+++
title = "A post united with a square block inscribed in it, flush at both ends"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "planar", "tangent-line", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/S2"

[volume]
value = 785.39816339744831
exact = "V = 10·25π (the block lies inside the post)"
tex = '10\cdot 25\pi'

[area]
value = 471.23889803846899
exact = "area = 2·25π + 2π·5·10 (the union is the post)"
tex = '2\cdot 25\pi + 2\pi\cdot 5\cdot 10'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block of square section, x and y from −a to a with a = 5/√2 ≈ 3.5355, and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Make the union of the post and the block.

The block's four vertical edges, through (±a, ±a), lie on the post's wall: the block is inscribed in the post. Its sides are inside the post, which makes the union the post itself; the block's top and bottom faces lie in the post's.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \bigl([-a,a]^2\times[0,10]\bigr),\quad a = 5/\sqrt2$$
