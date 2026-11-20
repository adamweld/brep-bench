+++
title = "A cylinder less a block with a corner on its axis"
tags = ["boolean", "cut", "analytic", "coincident", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/ZC1"

[volume]
value = 351.85837720205683
exact = "cylinder π·4²·8 less a quarter of the disk of radius 4 (4π) times the block's height 4"
tex = '128\pi - 4\cdot 4\pi'

[area]
value = 308.4601535159018
exact = "cylinder 96π; the base's loss and the cut floor's gain at z = 4 cancel; the wall loses a quarter turn, (π/2)·4·4 = 8π; the new walls x = 0 and y = 0 are each 4 × 4"
tex = '96\pi - 8\pi + 32'
+++

A cylinder of radius 4 and height 8 standing on the xy plane with its axis the z axis, and a block from (0, 0, 0) to (5, 5, 4): half the cylinder's height, its base in the cylinder's base. Cut the block away from the cylinder.

One vertical edge of the block is the cylinder's axis, so two of its faces, x = 0 and y = 0, pass through the axis. Of the block's corners only (0, 0) lies inside the cylinder's circle, and its other faces, x = 5 and y = 5, lie clear of the cylinder. What is left is three quarters of the cylinder up to z = 4 and all of it above.

**As a set:**

$$\{\rho \le 4,\ 0 \le z \le 8\} \setminus \bigl([0,5]\times[0,5]\times[0,4]\bigr)$$
