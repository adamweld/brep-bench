+++
title = "A counterbored hole through a block"
tags = ["hole", "counterbore", "cut", "planar", "cylinder", "analytic"]

[volume]
value = 23033.960259021139
exact = "40·30·20 − π·3²·20 − π(5.5² − 3²)·6 = 24000 − 307.5π"
tex = '40\cdot 30\cdot 20 - \pi\cdot 3^2\cdot 20 - \pi\left(5.5^2 - 3^2\right)\cdot 6 = 24000 - 307.5\pi'

[area]
value = 5614.6902302738527
exact = "5200 − π·5.5² − π·3² + π(5.5² − 3²) + 2π·5.5·6 + 2π·3·14 = 5200 + 132π: the block, less the mouths top and bottom, and the counterbore's floor, its wall and the hole's wall"
tex = '5200 - \pi\cdot 5.5^2 - \pi\cdot 3^2 + \pi\left(5.5^2 - 3^2\right) + 2\pi\cdot 5.5\cdot 6 + 2\pi\cdot 3\cdot 14 = 5200 + 132\pi'
+++

A block, the box from the origin to (40, 30, 20), with a counterbored hole through it, its axis
the vertical line through (20, 15).

- The hole: radius 3, right through the block from the top (z = 20) to the bottom (z = 0).
- The counterbore: radius 5.5 and 6 deep, from the top face down to a flat floor at z = 14, square
  to the axis. So the hole's wall of radius 3 runs from z = 0 to z = 14, and the floor is the ring
  between radii 3 and 5.5 at z = 14.

If you cut the hole with a cylinder, make it run past the bottom face.

**As a set:**

$$\bigl([0,40]\times[0,30]\times[0,20]\bigr) \setminus \bigl(\{\,d \le 3\,\} \cup \{\,d \le 5.5,\ z \ge 14\,\}\bigr),\quad d = \sqrt{(x-20)^2 + (y-15)^2}$$
