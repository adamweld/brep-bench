+++
title = "A block less the same block with a blind hole: the plug of the hole"
tags = ["boolean", "cut", "analytic", "cylinder", "planar", "coincident", "hole"]
origin = "OCCT boolean/bcut_simple/M3"

[volume]
value = 50.265482457436692
exact = "πr²·4 = 16π, r = 2"
tex = '4\pi r^2 = 16\pi,\quad r = 2'

[area]
value = 75.398223686155038
exact = "2πr·4 + 2πr² = 24π, r = 2"
tex = '2\pi r\cdot 4 + 2\pi r^2 = 24\pi'
+++

A block from (0, 0, 0) to (10, 10, 6), and a second block, the same block with a blind hole: the block less the cylinder of radius 2 about the axis parallel to z through (5, 5), from z = −1 up to z = 4, so the hole opens in the bottom face and ends flat 4 up. Cut the holed block from the plain block. All their faces coincide except the hole; what is left is the plug that filled it: the cylinder of radius 2 about that axis from z = 0 to z = 4.

**As a set:**

$$[0,10]^2\times[0,6] \setminus \bigl([0,10]^2\times[0,6] \setminus \{(x-5)^2 + (y-5)^2 \le 4,\ z \le 4\}\bigr)$$
