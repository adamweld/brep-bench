+++
title = "A cylinder less an equal cylinder raised half its height"
tags = ["boolean", "cut", "analytic", "cylinder", "coincident"]
origin = "OCCT boolean/bcut_simple/L8"

[volume]
value = 392.69908169872415
exact = "πr²·5 = 125π, r = 5"
tex = '5\pi r^2 = 125\pi,\quad r = 5'

[area]
value = 314.15926535897932
exact = "2πr·5 + 2πr² = 100π, r = 5"
tex = '2\pi r\cdot 5 + 2\pi r^2 = 100\pi'
+++

Two cylinders of radius 5 on the z axis, the first from z = 0 to z = 10 and the second from z = 5 to z = 15. Cut the second from the first. Between z = 5 and z = 10 their walls coincide. What is left is the cylinder from z = 0 to z = 5: its top is the plane z = 5 where the second's bottom was, and its wall is the first's wall below that. If the cylinders you make have a seam, turn the second a quarter turn about the z axis, so that the coincident walls are not split in the same place.

**As a set:**

$$\{\rho \le 5,\ 0 \le z \le 10\} \setminus \{\rho \le 5,\ 5 \le z \le 15\}$$
