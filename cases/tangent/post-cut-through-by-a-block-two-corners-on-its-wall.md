+++
title = "A post cut through by a block, two corners on its wall"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/V2"

[volume]
value = 264.52288619912721
exact = "V = 10 (25π − A), where A = 25·arcsin(3/5) + 36 is the area of the disc of radius 5 inside the block's outline"
tex = '10\,(25\pi - A),\quad A = 25 \operatorname{asin}{\left(\frac{3}{5} \right)} + 36'

[area]
value = 522.71373171947633
exact = "area = 2 (25π − A) + 10 (10π − a) + 10 ℓ, where a = -10·arctan(4/3) + 5·π is the length of the post's circle inside the block's outline and ℓ = 22 is the length of the outline's sides inside the disc"
tex = '2\,(25\pi - A) + 10\,(10\pi - a) + 10\,\ell,\quad a = - 10 \operatorname{atan}{\left(\frac{4}{3} \right)} + 5 \pi,\ \ell = 22'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −3 to 3, y from −4 to 7 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Cut the block away from the post.

Two corners, (±3, −4), lie exactly on the post's wall (3² + 4² = 5²); the other two, (±3, 7), are outside. No face of the block is tangent to the post. The post's axis is inside the block's outline.

What is left is the part of the post with x < −3 or x > 3 or y < −4. The part left can be in several pieces, even pieces that touch only along a line; build it all as one result, and measure it whole.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \setminus \bigl([-3,3]\times[-4,7]\times[0,10]\bigr)$$
