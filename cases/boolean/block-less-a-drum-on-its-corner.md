+++
title = "A block less a drum standing on its corner"
tags = ["boolean", "cut", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/boptuc_simple/ZC1"

[volume]
value = 81.82522957531896
exact = "height 5 times the block's footprint less the quarter disc of the drum in it: 5(6² − πr²/4), r = 5"
tex = '5\left(6^2 - \tfrac{\pi r^2}{4}\right),\quad r = 5'

[area]
value = 142
exact = "top and bottom 2(36 − 25π/4); outer faces x = 6 and y = 6, 2·6·5 = 60; the strips of the faces x = 0 and y = 0 left outside the drum, 2·1·5 = 10; the drum's wall, a quarter circle of length 5π/2 by height 5, 25π/2. The π terms cancel"
tex = '2\left(36 - \tfrac{25\pi}{4}\right) + 60 + 10 + \tfrac{25\pi}{2} = 142'
+++

A block with x from 0 to 6, y from 0 to 6 and z from 0 to 5, and a drum: a cylinder of radius 5 with the z axis as its axis, from z = 0 to z = 10. The drum's axis runs along the block's vertical edge at the origin, so the drum covers a quarter of the block's section, and its base is level with the block's base. Cut the drum away from the block.

What is left is the part of the block outside the drum, from the base to the block's top, in one piece: a crescent wrapped around the quarter circle, with the block's far corner at (6, 6) at its middle. Its inner wall is a quarter of the drum's wall, concave, a quarter circle by 5 high.

**As a set:**

$$\bigl([0,6]\times[0,6]\times[0,5]\bigr) \setminus \{\,\rho \le 5,\ 0 \le z \le 10\,\}$$
