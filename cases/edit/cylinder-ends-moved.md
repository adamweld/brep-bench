+++
title = "A cylinder's ends moved, the top out and the bottom in"
tags = ["edit", "move-face", "cylinder", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/A1, OCCT tests/offset/faces_type_i/A2, OCCT tests/offset/faces_type_i/A3, OCCT tests/offset/faces_type_i/A4"

[volume]
value = 863.93797973719314
exact = "π·5²·11 = 275π"
tex = '\pi\cdot 5^2\cdot 11 = 275\pi'

[area]
value = 502.65482457436692
exact = "2π·5² + 2π·5·11 = 160π"
tex = '2\pi\cdot 5^2 + 2\pi\cdot 5\cdot 11 = 160\pi'
+++

A cylinder of radius 5 about the z axis, from the flat bottom in the plane z = 0 to the flat top in
the plane z = 10. Its faces are the side and two flat disks.

Then edit the solid directly: move the top 3 outward along its normal, up to the plane z = 13, and
the bottom 2 inward along its normal, up to the plane z = 2. The side runs on to meet them.

So the result is the cylinder of radius 5 from z = 2 to z = 13.

**As a set:**

$$\{\, \rho \le 5,\ 2 \le z \le 13 \,\}$$
