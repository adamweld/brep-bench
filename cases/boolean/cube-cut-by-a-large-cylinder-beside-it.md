+++
title = "A cube cut by a large cylinder whose axis lies outside it"
tags = ["boolean", "cut", "analytic", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/ZO6"

[volume]
value = 543.3885225094818
exact = "10·(100 − S), S = 25√3 + 50π/3 − 50 the area of the cube's square within the circle"
tex = '10\,(100 - S),\quad S = 25\sqrt3 + \tfrac{50\pi}{3} - 50'

[area]
value = 440.1923788646684
exact = "2(100 − S) + 10·(10 + 2(15 − 5√3) + 10π/3): top and bottom, then the unrolled boundary of the square less the circle: the back edge, two side edges 15 − 5√3 long, an arc of 60° of a circle of radius 10"
tex = '2(100 - S) + 10\left(40 - 10\sqrt3 + \tfrac{10\pi}{3}\right)'
+++

A cube of side 10 between the origin and (10, 10, 10), less a cylinder of radius 10 and height 10 whose axis is the vertical line through (5, −5), from z = 0 to z = 10: the axis lies 5 outside the cube, beside the middle of its face y = 0, and the cylinder's ends are in the cube's top and bottom faces.

The cylinder covers the whole of the face y = 0 (its corners (0, 0) and (10, 0) are 5√2 from the axis, inside the cylinder) and a strip of the cube behind it, 5 deep in the middle and 5√3 − 5 ≈ 3.66 deep at the faces x = 0 and x = 10, where the cylinder's wall crosses those faces. What is left is the cube with a shallow round-bottomed scoop taken out of one side, running the full height.

**As a set:**

$$[0,10]^3 \setminus \{(x-5)^2 + (y+5)^2 \le 10^2\}$$
