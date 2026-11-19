+++
title = "A drum domed by a ball whose poles lie on its rim"
tags = ["tangent", "boolean", "union", "analytic", "sphere", "pole", "seam", "curve-contact"]
origin = "OCCT tests/boolean/bopfuse_simple/ZH9"

[volume]
value = 890.1179185171081
exact = "πR²H + 2πR³/3 = 200π + 250π/3 = 850π/3, R = 5, H = 8"
tex = '\pi R^2 H + \tfrac{2}{3}\pi R^3 = \tfrac{850\pi}{3},\quad R = 5,\ H = 8'

[area]
value = 486.94686130641793
exact = "2πRH + πR² + 2πR² = 80π + 25π + 50π = 155π"
tex = '2\pi R H + \pi R^2 + 2\pi R^2 = 155\pi'
+++

A cylinder (a drum) of radius 5 on the z axis, from z = 0 to z = 8, united with a ball of the same
radius centered at (0, 0, 8), the middle of the drum's top.

The ball's great circle in the plane z = 8 is the drum's top rim, and along it the sphere touches
the drum's wall, tangent to it. The lower half of the ball lies inside the drum, and the upper half
is a dome on its top. The union is the drum capped by a hemisphere: the wall runs on into the dome
without a crease where the rim was, and the drum's top face is gone.

This case is about where the sphere's poles lie. Put the ball's poles at (5, 0, 8) and (−5, 0, 8),
on the rim, its axis through them parallel to the x axis; and if your ball has a seam, put it in
the plane z = 8, on the half of the rim where y > 0. The circle along which the two solids meet is
then a meridian through both poles, half of it the seam.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 8\,\} \cup B_5(0, 0, 8)$$
