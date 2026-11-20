+++
title = "A post and a block half as tall around it, tangent on three sides, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcommon_simple/W1"

[volume]
value = 392.699081698724155
exact = "V = 125π: the common part is the post's lower half, a cylinder of radius 5 and height 5"
tex = '125\pi'

[area]
value = 314.159265358979324
exact = "area = 100π: the cylinder's two discs 2·25π and its wall 2π·5·5"
tex = '100\pi'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −5 to 9, y from −5 to 5 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base, its top face at z = 5 inside the post. Take the part common to the post and the block.

The post lies wholly inside the block's outline, whose faces x = −5, y = −5 and y = 5 are each tangent to the post's wall along a vertical line; the face x = 9 is clear of it. No corner of the block is inside or on the post.

What is common is the post's lower half: a cylinder of radius 5 and height 5, a new top disc at z = 5.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cap \bigl([-5,9]\times[-5,5]\times[0,5]\bigr)$$
