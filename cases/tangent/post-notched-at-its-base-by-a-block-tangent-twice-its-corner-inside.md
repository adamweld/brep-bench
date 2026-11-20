+++
title = "A post notched at its base by a block, tangent twice, its corner inside"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar", "tangent-line"]
origin = "OCCT tests/boolean/bopcut_simple/X7"

[volume]
value = 501.78575437360672
exact = "V = 250π − 5A, where A = -25·arcsin(4/5)/2 + 25·arcsin(3/5)/2 + 21 + 25·π/2 is the area of the disc of radius 5 inside the block's outline"
tex = '250\pi - 5A,\quad A = - \frac{25 \operatorname{asin}{\left(\frac{4}{5} \right)}}{2} + \frac{25 \operatorname{asin}{\left(\frac{3}{5} \right)}}{2} + 21 + \frac{25 \pi}{2}'

[area]
value = 469.79393442893235
exact = "area = 150π − 5a + 5ℓ (the post's 2π·5² + 2π·5·10, less the wall the block covers, plus the notch's walls), where a = -5·arctan(4/3) + 5·arctan(3/4) + 5·π is the length of the post's circle inside the block's outline and ℓ = 14 is the length of the outline's sides inside the disc"
tex = '150\pi - 5a + 5\ell,\quad a = - 5 \operatorname{atan}{\left(\frac{4}{3} \right)} + 5 \operatorname{atan}{\left(\frac{3}{4} \right)} + 5 \pi,\ \ell = 14'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −3 to 5, y from −3 to 5 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base. Cut the block away from the post. The block's top face at z = 5 is inside the post, so the cut makes a notch at the post's base and the post is whole above z = 5.

The block's faces x = 5 and y = 5 each touch the post's wall along a line, where block and post are tangent. Its corner (−3, −3) is inside the post, about 4.24 from the axis; its other three corners are outside. The post's axis is inside the block's outline.

Below z = 5 what is left is the part of the post with x < −3 or y < −3; above z = 5 the post is whole. The part left can be in several pieces, even pieces that touch only along a line; build it all as one result, and measure it whole.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \setminus \bigl([-3,5]\times[-3,5]\times[0,5]\bigr)$$
