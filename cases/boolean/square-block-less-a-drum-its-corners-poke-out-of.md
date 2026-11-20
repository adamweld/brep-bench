+++
title = "A square block less a drum that its corners poke out of"
tags = ["boolean", "cut", "analytic", "cylinder", "planar", "pieces"]
origin = "OCCT tests/boolean/boptuc_simple/ZB7"

[volume]
value = 9.051472697918039
exact = "height 5 times the four corners of the square outside the disc: 5(16 − 25π + 100θ), θ = arctan(3/4) = arccos(4/5). The disc inside the square is 25π less four segments, each 25θ − 12"
tex = '5\left(16 - 25\pi + 100\theta\right),\quad \theta = \arctan\tfrac34'

[area]
value = 72
exact = "top and bottom 2(16 − 25π + 100θ); the eight strips of the block's four side faces left outside the drum, each 1 long by 5 high, 40; the drum's wall in four arcs, each of angle π/2 − 2θ and radius 5, 5 high, 4·25(π/2 − 2θ). The π and θ terms cancel"
tex = '2\left(16 - 25\pi + 100\theta\right) + 40 + 100\left(\tfrac{\pi}{2} - 2\theta\right) = 72'
+++

A block with x from −4 to 4, y from −4 to 4 and z from 0 to 5, and a drum: a cylinder of radius 5 with the z axis as its axis, from z = 0 to z = 10. The block's four side faces are inside the drum, at distance 4 from the axis, but its four vertical edges are at distance 4√2, about 5.66, so they poke out through the drum's wall. The block's base and the drum's base are level. Cut the drum away from the block.

What is left is the four corners of the block outside the drum: four separate pieces, one at each vertical edge, each bounded by two strips of the block's faces, 1 long by 5 high, the concave wall of the drum, and a flat top and bottom. Build it all as one result and measure it whole.

**As a set:**

$$\bigl([-4,4]\times[-4,4]\times[0,5]\bigr) \setminus \{\,\rho \le 5,\ 0 \le z \le 10\,\}$$
