+++
title = "A cube and a cylinder with its axis outside it, their common part"
tags = ["boolean", "common", "analytic", "cylinder", "coincident"]
origin = "OCCT tests/boolean/bopcommon_simple/ZO6"

[volume]
value = 456.6114774905182
exact = "10·S with S = 25√3 + 50π/3 − 50 the area of the cube's square within the circle"
tex = '10\,S,\quad S = 25\sqrt3 + \tfrac{50\pi}{3} - 50'

[area]
value = 369.24713137465114
exact = "2S + 10·(10√3 + 10π/3): top and bottom, then the unrolled boundary of the square within the circle: the edge y = 0 (10), two side edges 5√3 − 5 long, an arc of 60° of a circle of radius 10"
tex = '2S + 10\left(10\sqrt3 + \tfrac{10\pi}{3}\right)'
+++

A cube of side 10 between the origin and (10, 10, 10), and a cylinder of radius 10 and height 10 whose axis is the vertical line through (5, −5), from z = 0 to z = 10: the axis lies 5 outside the cube, beside the middle of its face y = 0, and the cylinder's ends are level with the cube's top and bottom. Take the part common to the two.

The cylinder covers the whole of the face y = 0 (its corners (0, 0) and (10, 0) are 5√2 from the axis, inside) and a strip of the cube behind it, 5 deep in the middle and 5√3 − 5 ≈ 3.66 deep at the faces x = 0 and x = 10, where the cylinder's wall crosses them. The common part is that strip: a slab of the cube, full height, bounded in front by the cube's face y = 0 and behind by a round face of the cylinder's wall, bulging away from the cube.

**As a set:**

$$[0,10]^3 \cap \{(x-5)^2 + (y+5)^2 \le 10^2\}$$
