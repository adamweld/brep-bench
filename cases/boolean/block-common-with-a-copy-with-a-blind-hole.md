+++
title = "A block and the same block with a blind hole, their common part"
tags = ["boolean", "common", "analytic", "cylinder", "planar", "coincident", "hole"]
origin = "OCCT boolean/bcommon_simple/J2"

[volume]
value = 549.7345175425633
exact = "10·10·6 − πr²·4 = 600 − 16π, r = 2"
tex = '600 - 4\pi r^2 = 600 - 16\pi'

[area]
value = 490.2654824574367
exact = "the block's 2·100 + 4·60 = 440, less the opening's disc 4π in the bottom, plus the hole's floor 4π and wall 2π·2·4 = 16π: 440 + 16π"
tex = '440 - 4\pi + 4\pi + 16\pi = 440 + 16\pi'
+++

A block from (0, 0, 0) to (10, 10, 6), and a second block, the same block with a blind hole: the block less the cylinder of radius 2 about the axis parallel to z through (5, 5), from z = −1 up to z = 4, so the hole opens in the bottom face and ends flat 4 up. Make the part common to the plain block and the holed block.

All the faces of the two coincide except the hole, which only the holed block has, so what is common is the holed block itself: the block with the blind hole.

**As a set:**

$$[0,10]^2\times[0,6] \cap \bigl([0,10]^2\times[0,6] \setminus \{(x-5)^2 + (y-5)^2 \le 4,\ z \le 4\}\bigr)$$
