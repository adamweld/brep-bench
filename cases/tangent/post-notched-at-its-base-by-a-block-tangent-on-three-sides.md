+++
title = "A post notched at its base by a block, tangent on three sides"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar", "tangent-line"]
origin = "OCCT tests/boolean/bopcut_simple/V7"

[volume]
value = 491.78325984009183
exact = "V = 250π − 5A, where A = -25·arcsin(√(21)/5) + 2·√(21) + 25·π is the area of the disc of radius 5 inside the block's outline"
tex = '250\pi - 5A,\quad A = - 25 \operatorname{asin}{\left(\frac{\sqrt{21}}{5} \right)} + 2 \sqrt{21} + 25 \pi'

[area]
value = 417.94899634490815
exact = "area = 150π − 5a + 5ℓ (the post's 2π·5² + 2π·5·10, less the wall the block covers, plus the notch's walls), where a = 10·arctan(2·√(21)/21) + 5·π is the length of the post's circle inside the block's outline and ℓ = 2·√(21) is the length of the outline's sides inside the disc"
tex = '150\pi - 5a + 5\ell,\quad a = 10 \operatorname{atan}{\left(\frac{2 \sqrt{21}}{21} \right)} + 5 \pi,\ \ell = 2 \sqrt{21}'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −5 to 5, y from −5 to 2 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base. Cut the block away from the post. The block's top face at z = 5 is inside the post, so the cut makes a notch at the post's base and the post is whole above z = 5.

The block's faces x = ±5 and y = −5 each touch the post's wall along a line, where block and post are tangent. Its face y = 2 cuts across the post. All four corners are outside the post. The post's axis is inside the block's outline.

Below z = 5 what is left is the part of the post with y > 2; above z = 5 the post is whole. The part left can be in several pieces, even pieces that touch only along a line; build it all as one result, and measure it whole.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \setminus \bigl([-5,5]\times[-5,2]\times[0,5]\bigr)$$
