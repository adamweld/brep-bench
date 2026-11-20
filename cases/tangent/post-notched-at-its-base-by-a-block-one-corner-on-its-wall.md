+++
title = "A post notched at its base by a block, one corner on its wall"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/ZB3"

[volume]
value = 489.48626114724678
exact = "V = 250π − 5A, where A = 25·arcsin(4/5) + 36 is the area of the disc of radius 5 inside the block's outline"
tex = '250\pi - 5A,\quad A = 25 \operatorname{asin}{\left(\frac{4}{5} \right)} + 36'

[area]
value = 524.87413713838837
exact = "area = 150π − 5a + 5ℓ (the post's 2π·5² + 2π·5·10, less the wall the block covers, plus the notch's walls), where a = -10·arctan(3/4) + 5·π is the length of the post's circle inside the block's outline and ℓ = 20 is the length of the outline's sides inside the disc"
tex = '150\pi - 5a + 5\ell,\quad a = - 10 \operatorname{atan}{\left(\frac{3}{4} \right)} + 5 \pi,\ \ell = 20'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −4 to 3, y from −5 to 4 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base. Cut the block away from the post. The block's top face at z = 5 is inside the post, so the cut makes a notch at the post's base and the post is whole above z = 5.

One corner, (3, 4), lies exactly on the post's wall (3² + 4² = 5²); the other three, (−4, 4), (3, −5) and (−4, −5), are outside. No face of the block is tangent to the post. The post's axis is inside the block's outline.

Below z = 5 what is left is the part of the post with x < −4 or x > 3 or y > 4; above z = 5 the post is whole. The part left can be in several pieces, even pieces that touch only along a line; build it all as one result, and measure it whole.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \setminus \bigl([-4,3]\times[-5,4]\times[0,5]\bigr)$$
