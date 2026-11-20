+++
title = "A cylinder less an equal cylinder whose axis lies on its wall"
tags = ["boolean", "cut", "analytic", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/ZF2"

[volume]
value = 103.31403956897597
exact = "height 6 times the disk less the lens: π·3² − L, L = r²(2π/3 − √3/2) the overlap of two disks of radius r = 3 whose centres are r apart"
tex = '6\left(\pi r^2 - r^2\left(\tfrac{2\pi}{3} - \tfrac{\sqrt3}{2}\right)\right),\quad r = 3'

[area]
value = 147.5353487188912
exact = "the two ends 2(π·3² − L); the wall of the first outside the second, an arc of 4π/3 of radius 3, height 6; and the second's wall inside the first, an arc of 2π/3, height 6: together 2π·3·6"
tex = '2\left(\pi r^2 - L\right) + 2\pi r\cdot 6,\quad r = 3'
+++

A cylinder of radius 3 and height 6 standing on the xy plane with its axis the z axis, and a second cylinder of the same radius and height on the axis x = 3, y = 0, which lies on the first's wall. Both run from z = 0 to z = 6, so their ends are coplanar. Cut the second away from the first. What is left is the first cylinder with a lens-shaped bay taken out of its side.

**As a set:**

$$\{\rho \le 3,\ 0 \le z \le 6\} \setminus \{(x-3)^2 + y^2 \le 9,\ 0 \le z \le 6\}$$
