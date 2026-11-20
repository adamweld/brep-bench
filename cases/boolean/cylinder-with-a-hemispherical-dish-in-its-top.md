+++
title = "A cylinder with a hemispherical dish in the middle of its top"
tags = ["boolean", "cut", "analytic", "sphere"]
origin = "OCCT tests/boolean/bopcut_simple/ZI4"

[volume]
value = 284.8377339254746
exact = "π·4²·6 less a hemisphere, (2/3)π·2³"
tex = '\pi r^2 h - \tfrac23\pi s^3 = 96\pi - \tfrac{16\pi}{3}'

[area]
value = 263.89378290154264
exact = "wall 2π·4·6, base π·4², the top's remaining annulus π(4² − 2²), the dish 2π·2²"
tex = '2\pi r h + \pi r^2 + \pi(r^2 - s^2) + 2\pi s^2 = 84\pi'
+++

A cylinder of radius 4 and height 6 standing on the xy plane with its axis the z axis, and a ball of radius 2 centred at (0, 0, 6), the middle of the cylinder's top disc. Cut the ball away from the cylinder: a hemispherical dish of radius 2 in the top.

**As a set:**

$$\{\rho \le 4,\ 0 \le z \le 6\} \setminus B_2\bigl((0, 0, 6)\bigr)$$
