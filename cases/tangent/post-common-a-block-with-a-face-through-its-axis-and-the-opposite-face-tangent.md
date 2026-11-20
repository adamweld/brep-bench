+++
title = "A post and a block with one face through its axis and the opposite face tangent, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cylinder", "planar", "tangent-line"]
origin = "OCCT boolean/bcommon_simple/H8"

[volume]
value = 196.34954084936207
exact = "5·(25π/2): the half disc of radius 5 under the post, 5 high"
tex = '5\cdot\tfrac{25\pi}{2} = \tfrac{125\pi}{2}'

[area]
value = 207.07963267948966
exact = "two half discs of 25π/2, the flat face y = 0 of 10·5 and half the post's wall π·5·5: 25π + 50 + 25π"
tex = '25\pi + 50 + 25\pi = 50 + 50\pi'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A block with x from −7 to 7, y from 0 to 5 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base, its top face at z = 5 inside the post. Make the part common to the post and the block.

The block's face y = 0 passes through the post's axis, and its opposite face y = 5 touches the post's wall along the vertical line through (0, 5). Its other two faces, x = ±7, are clear of the post. What is left is the post's half with y ≥ 0, up to z = 5: a half cylinder with a flat back on y = 0, two flat half discs at its ends and half of the post's wall.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cap \bigl([-7,7]\times[0,5]\times[0,5]\bigr)$$
