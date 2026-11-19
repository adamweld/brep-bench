+++
title = "A cylinder made of two cylinders joined end to end"
tags = ["boolean", "fuse", "analytic", "coincident", "cylinder"]
origin = "OCCT heal/bug33421"

[volume]
value = 1570.7963267948967
exact = "π·5²·20"
tex = '\pi\cdot 5^2\cdot 20'

[area]
value = 785.3981633974483
exact = "2π·5·20 + 2π·5²"
tex = '2\pi\cdot 5\cdot 20 + 2\pi\cdot 5^2'
+++

Two cylinders of radius 5 and height 10, both about the z axis: the first from z = 0 up to z = 10, the second from z = 0 down to z = −10, so that the second is the first mirrored across the plane z = 0 and the two share their disc at z = 0. Unite them. The result is the one cylinder of radius 5 from z = −10 to z = 10.

**As a set:**

$$\bigl(\{\rho\le5\}\times[0,10]\bigr)\cup\bigl(\{\rho\le5\}\times[-10,0]\bigr)$$
