+++
title = "A post less the drum it passes through"
tags = ["boolean", "cut", "analytic", "cylinder", "pieces"]
origin = "OCCT tests/boolean/boptuc_simple/ZD3"

[volume]
value = 75.39822368615503
exact = "two stubs of the post, each a cylinder of radius 2 and height 3: 2·π·2²·3"
tex = '2\cdot \pi r^2 h,\quad r = 2,\ h = 3'

[area]
value = 125.66370614359172
exact = "two cylinders of radius 2 and height 3, each a wall and two flat ends: 2(2π·2·3 + 2π·2²)"
tex = '2\left(2\pi r h + 2\pi r^2\right),\quad r = 2,\ h = 3'
+++

A drum: a cylinder of radius 4 with the z axis as its axis, from z = 0 to z = 6. A post: a narrower and longer cylinder of radius 2 on the same axis, from z = −3 to z = 9, so that it passes through both end faces of the drum. Cut the drum away from the post.

What is left is the two ends of the post that stand out of the drum: two separate cylinders of radius 2 and height 3, one from z = −3 to z = 0 and one from z = 6 to z = 9. Build it all as one result and measure it whole.

**As a set:**

$$\{\,\rho \le 2,\ -3 \le z \le 9\,\} \setminus \{\,\rho \le 4,\ 0 \le z \le 6\,\}$$
