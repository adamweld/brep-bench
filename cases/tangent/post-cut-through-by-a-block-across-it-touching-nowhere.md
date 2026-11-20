+++
title = "A post cut through by a block, across it, touching nowhere"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar", "tangent-line"]
origin = "OCCT tests/boolean/bopcut_simple/ZA4"

[volume]
value = 163.50110879328439
exact = "V = 10 (25π − A), where A = -50·arcsin(3/5) + 50·arcsin(4/5) + 48 is the area of the disc of radius 5 inside the block's outline"
tex = '10\,(25\pi - A),\quad A = - 50 \operatorname{asin}{\left(\frac{3}{5} \right)} + 50 \operatorname{asin}{\left(\frac{4}{5} \right)} + 48'

[area]
value = 530.10066527597063
exact = "area = 2 (25π − A) + 10 (10π − a) + 10 ℓ, where a = -20·arctan(3/4) + 20·arctan(4/3) is the length of the post's circle inside the block's outline and ℓ = 24 is the length of the outline's sides inside the disc"
tex = '2\,(25\pi - A) + 10\,(10\pi - a) + 10\,\ell,\quad a = - 20 \operatorname{atan}{\left(\frac{3}{4} \right)} + 20 \operatorname{atan}{\left(\frac{4}{3} \right)},\ \ell = 24'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −4 to 4, y from −4 to 4 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Cut the block away from the post.

All four corners, (±4, ±4), are outside the post (about 5.66 from the axis), and each face of the block cuts across the post. Nothing is tangent and no corner is on the wall. The post's axis is inside the block's outline.

What is left is the part of the post with x < −4 or x > 4 or y < −4 or y > 4. The part left can be in several pieces, even pieces that touch only along a line; build it all as one result, and measure it whole.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \setminus \bigl([-4,4]\times[-4,4]\times[0,10]\bigr)$$
