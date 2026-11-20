+++
title = "A cube with a ball bulging through its faces"
tags = ["boolean", "union", "analytic", "sphere", "planar"]
origin = "OCCT tests/bugs/moddata_1/bug217"

[volume]
value = 1106.814150222053
exact = "1000 + 6·πh²(3R − h)/3, R = 6, h = R − 5 = 1: 1000 + 34π"
tex = '10^3 + 6\cdot\tfrac{\pi h^2(3R - h)}{3} = 1000 + 34\pi,\quad R = 6,\ h = 1'

[area]
value = 618.8495559215388
exact = "600 − 6π(R² − 5²) + 6·2πRh = 600 − 66π + 72π"
tex = '600 - 6\pi\left(R^2 - 25\right) + 6\cdot 2\pi R h = 600 + 6\pi'
+++

A cube of side 10 centered on the origin, and a ball of radius 6 centered on the origin too. Unite
them.

The ball is wider than the cube's faces are far apart (6 > 5) but does not reach its edges
(6 < 5√2), so it bulges through each face as a round cap of height 1 and a rim of radius √11, and
the cube's eight corners stick out of the ball. The six caps do not touch one another.

**As a set:**

$$[-5,5]^3 \cup B_6$$
