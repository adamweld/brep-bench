+++
title = "A cylinder shrunk by 2"
tags = ["offset", "shrink", "analytic"]
origin = "OCCT tests/offset/shape_type_i/A2"

[volume]
value = 113.09733552923255
exact = "π(R − d)²(h − 2d), R = 5, h = 8, d = 2: 36π"
tex = '\pi (R - d)^2 (h - 2d) = 36\pi'

[area]
value = 131.94689145077132
exact = "2π(R − d)² + 2π(R − d)(h − 2d) = 18π + 24π = 42π"
tex = '2\pi (R - d)^2 + 2\pi (R - d)(h - 2d) = 42\pi'
+++

A cylinder of radius 5 on the z axis, from z = 0 to z = 8, shrunk by 2: the solid of every point
at least 2 from its outside.

The side moves in to radius 3, and the ends move in to z = 2 and z = 6, so the result is the
cylinder of radius 3 from z = 2 to z = 6.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 8\,\} \ominus B_{2}$$
