+++
title = "A post and a block inscribed in its circle, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcommon_simple/U9"

[volume]
value = 480.0
exact = "V = 10·A, where A = 48 is the area of the block's outline, which lies inside the post's disc"
tex = '10A,\quad A = 48'

[area]
value = 376.0
exact = "area = 2A + 10(a + ℓ), where a = 0 is the length of the post's circle inside the block's outline and ℓ = 28 is the perimeter of the outline"
tex = '2A + 10(a + \ell),\quad a = 0,\ \ell = 28'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −3 to 3, y from −4 to 4 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Take the part common to the post and the block.

All four of the block's vertical edges, at (±3, ±4), lie exactly on the post's wall (3² + 4² = 5²): the block is inscribed in the post, the post's axis at its centre. No face is tangent to the post.

The block lies wholly inside the post, so what is common is the block itself, touching the post's wall along its four vertical edges.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cap \bigl([-3,3]\times[-4,4]\times[0,10]\bigr)$$
