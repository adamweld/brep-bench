+++
title = "A post notched at its base by a block, tangent once, its corners clear"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar", "tangent-line"]
origin = "OCCT tests/boolean/bopcut_simple/Y5"

[volume]
value = 454.01199749620580
exact = "V = 250π − 5A, where A = -50·arcsin(3/5) + 25·arcsin(4/5) + 36 + 25·π/2 is the area of the disc of radius 5 inside the block's outline"
tex = '250\pi - 5A,\quad A = - 50 \operatorname{asin}{\left(\frac{3}{5} \right)} + 25 \operatorname{asin}{\left(\frac{4}{5} \right)} + 36 + \frac{25 \pi}{2}'

[area]
value = 500.68443167797198
exact = "area = 150π − 5a + 5ℓ (the post's 2π·5² + 2π·5·10, less the wall the block covers, plus the notch's walls), where a = -10·arctan(3/4) + 20·arctan(4/3) is the length of the post's circle inside the block's outline and ℓ = 18 is the length of the outline's sides inside the disc"
tex = '150\pi - 5a + 5\ell,\quad a = - 10 \operatorname{atan}{\left(\frac{3}{4} \right)} + 20 \operatorname{atan}{\left(\frac{4}{3} \right)},\ \ell = 18'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −5 to 4, y from −4 to 4 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base. Cut the block away from the post. The block's top face at z = 5 is inside the post, so the cut makes a notch at the post's base and the post is whole above z = 5.

The block's face x = −5 touches the post's wall along a line, where block and post are tangent. Its faces x = 4 and y = ±4 cut across the post. All four corners are outside the post, the nearest, (4, ±4), about 5.66 from the axis. The post's axis is inside the block's outline.

Below z = 5 what is left is the part of the post with x > 4 or y < −4 or y > 4; above z = 5 the post is whole. The part left can be in several pieces, even pieces that touch only along a line; build it all as one result, and measure it whole.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \setminus \bigl([-5,4]\times[-4,4]\times[0,5]\bigr)$$
