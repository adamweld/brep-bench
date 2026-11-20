+++
title = "A cylinder lofted between two circles, shrunk by 1"
tags = ["offset", "shrink", "loft", "cylinder", "analytic"]
origin = "OCCT tests/bugs/modalg_3/bug5805_57"

[volume]
value = 301.59289474462014
exact = "πr²h, r = 4, h = 6: 96π"
tex = '\pi r^2 h = 96\pi,\quad r = 4,\ h = 6'

[area]
value = 251.32741228718345
exact = "2πrh + 2πr² = 48π + 32π = 80π"
tex = '2\pi r h + 2\pi r^2 = 80\pi'
+++

A cylinder on the z axis made by a ruled loft between the circle of radius 5 at z = 0 and the
equal circle at z = 8, closed by the two disks they bound, shrunk by 1: the solid of every point at
least 1 from its outside.

Each face moves 1 inward: the side to radius 4, the base and the top to z = 1 and z = 7. The
shrunk cylinder has radius 4 and runs from z = 1 to z = 7.

**As a set:**

$$\{\, \rho \le 5,\ 0 \le z \le 8 \,\} \ominus B_{1}$$
