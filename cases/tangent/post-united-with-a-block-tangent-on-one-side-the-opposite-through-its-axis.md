+++
title = "A post united with a block, one face tangent and the opposite face through the axis, flush at both ends"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "planar", "tangent-line", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/U4"

[volume]
value = 1092.6990816987242
exact = "V = 10·|U|, |U| = 70 + 25π/2: the block's 70 plus the post's other half-disc"
tex = '10\,\bigl(70 + \tfrac{25\pi}{2}\bigr)'

[area]
value = 655.61944901923449
exact = "area = 2|U| + 10·P, P = 28 + 5π (the block's outline outside the disc: 14 + 5 + 5 + 2 + 2, and the disc's half circle with x < 0)"
tex = '2\,\bigl(70 + \tfrac{25\pi}{2}\bigr) + 10\,(28 + 5\pi)'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from 0 to 5, y from −7 to 7 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Make the union of the post and the block.

The block's face x = 5 touches the post's wall along a line, where block and post are tangent; its opposite face x = 0 passes through the post's axis. The faces y = ±7 are clear of the post, and the block runs past the post at both ends.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \bigl([0,5]\times[-7,7]\times[0,10]\bigr)$$
