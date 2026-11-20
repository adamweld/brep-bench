+++
title = "A post united with a bar that starts at its axis and leaves through its wall, flush at both ends"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "planar", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/ZA6"

[volume]
value = 910.8674379814595
exact = "V = 10·|U|, |U| = 32 + 25π − 2√21 − 25α, α = arcsin(2/5): the block's 32 plus the disc's 25π less the 2√21 + 25α where they overlap"
tex = '10\,\bigl(32 + 25\pi - 2\sqrt{21} - 25\alpha\bigr),\quad \alpha = \arcsin\tfrac25'

[area]
value = 563.52955444940562
exact = "area = 2|U| + 10·P, P = 20 − 2√21 + 10π − 10α (the block's outline outside the disc: 2·(8 − √21) + 4, and the disc's circle less the arc 10α inside the block)"
tex = '2|U| + 10\,\bigl(20 - 2\sqrt{21} + 10\pi - 10\alpha\bigr),\quad |U| = 32 + 25\pi - 2\sqrt{21} - 25\alpha'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from 0 to 8, y from −2 to 2 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Make the union of the post and the block.

The block's end face x = 0 lies in the plane through the post's axis, inside the post. The faces y = ±2 cross the post's wall at x = √21 ≈ 4.58, and the block runs out of the post, ending at x = 8 in a face clear of it.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \bigl([0,8]\times[-2,2]\times[0,10]\bigr)$$
