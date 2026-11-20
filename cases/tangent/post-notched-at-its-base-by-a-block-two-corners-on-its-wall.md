+++
title = "A post notched at its base by a block, two corners on its wall"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/Z6"

[volume]
value = 524.96052479828776
exact = "V = 250π − 5A, where A = 25·arcsin(3/5) + 36 is the area of the disc of radius 5 inside the block's outline"
tex = '250\pi - 5A,\quad A = 25 \operatorname{asin}{\left(\frac{3}{5} \right)} + 36'

[area]
value = 549.06384259880477
exact = "area = 150π − 5a + 5ℓ (the post's 2π·5² + 2π·5·10, less the wall the block covers, plus the notch's walls), where a = -10·arctan(4/3) + 5·π is the length of the post's circle inside the block's outline and ℓ = 22 is the length of the outline's sides inside the disc"
tex = '150\pi - 5a + 5\ell,\quad a = - 10 \operatorname{atan}{\left(\frac{4}{3} \right)} + 5 \pi,\ \ell = 22'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −3 to 3, y from −4 to 7 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base. Cut the block away from the post. The block's top face at z = 5 is inside the post, so the cut makes a notch at the post's base and the post is whole above z = 5.

Two corners, (±3, −4), lie exactly on the post's wall (3² + 4² = 5²); the other two, (±3, 7), are outside. No face of the block is tangent to the post. The post's axis is inside the block's outline.

Below z = 5 what is left is the part of the post with x < −3 or x > 3 or y < −4; above z = 5 the post is whole. The part left can be in several pieces, even pieces that touch only along a line; build it all as one result, and measure it whole.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \setminus \bigl([-3,3]\times[-4,7]\times[0,5]\bigr)$$
