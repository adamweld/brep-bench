+++
title = "A post united with a block with one corner on the post's wall and the opposite corner outside, flush at both ends"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "planar", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/ZA1"

[volume]
value = 1052.6990816987242
exact = "V = 10·|U|, |U| = 66 + 25π/2: the block's 90 plus the disc's 25π less the 25π/2 + 24 where they overlap (the disc less two circular segments, which add to a half disc less 24 because arccos(3/5) + arccos(4/5) = π/2)"
tex = '10\,\bigl(66 + \tfrac{25\pi}{2}\bigr)'

[area]
value = 607.61944901923449
exact = "area = 2|U| + 10·P, P = 24 + 5π (the block's outline outside the disc: 38 less 8 and 6 inside it, and the disc's arcs beyond the two faces, 10·arccos(3/5) + 10·arccos(4/5) = 5π)"
tex = '2\,\bigl(66 + \tfrac{25\pi}{2}\bigr) + 10\,(24 + 5\pi)'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −3 to 6, y from −4 to 6 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Make the union of the post and the block.

The block's vertical edge through (−3, −4) lies on the post's wall, since 3² + 4² = 5². From it the faces x = −3 and y = −4 run into the post, crossing it, and out through the wall again at (−3, 4) and (3, −4). The opposite corner (6, 6) and the other two, (−3, 6) and (6, −4), are outside the post.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \bigl([-3,6]\times[-4,6]\times[0,10]\bigr)$$
