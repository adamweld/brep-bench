+++
title = "A post cut through by a block, a face through its axis"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/ZA6"

[volume]
value = 590.86743798145950
exact = "V = 10 (25π − A), where A = -25·arcsin(√(21)/5) + 2·√(21) + 25·π/2 is the area of the disc of radius 5 inside the block's outline"
tex = '10\,(25\pi - A),\quad A = - 25 \operatorname{asin}{\left(\frac{\sqrt{21}}{5} \right)} + 2 \sqrt{21} + \frac{25 \pi}{2}'

[area]
value = 522.83258224763922
exact = "area = 2 (25π − A) + 10 (10π − a) + 10 ℓ, where a = 10·arctan(2·√(21)/21) is the length of the post's circle inside the block's outline and ℓ = 4 + 2·√(21) is the length of the outline's sides inside the disc"
tex = '2\,(25\pi - A) + 10\,(10\pi - a) + 10\,\ell,\quad a = 10 \operatorname{atan}{\left(\frac{2 \sqrt{21}}{21} \right)},\ \ell = 4 + 2 \sqrt{21}'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from 0 to 6, y from −2 to 2 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Cut the block away from the post.

The block's face x = 0 contains the post's axis, so the axis lies on the block's boundary, not inside it. Its corners (0, ±2) are inside the post and (6, ±2) are outside; no face is tangent to the post and no corner is on the wall.

What is left is the part of the post with x < 0 or y < −2 or y > 2. The part left can be in several pieces, even pieces that touch only along a line; build it all as one result, and measure it whole.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \setminus \bigl([0,6]\times[-2,2]\times[0,10]\bigr)$$
