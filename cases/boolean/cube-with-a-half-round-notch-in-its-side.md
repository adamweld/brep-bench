+++
title = "A cube with a half-round notch in the middle of one side"
tags = ["boolean", "cut", "analytic", "coincident"]
origin = "OCCT tests/boolean/bopcut_simple/ZN8"

[volume]
value = 901.825229575319
exact = "10³ − ½·π·2.5²·10"
tex = '10^3 - \tfrac12\pi\cdot 2.5^2\cdot 10'

[area]
value = 608.9048622548087
exact = "top and bottom 2(10² − ½π·2.5²), the faces x = 0, x = 10 and y = 10 whole (300), the face y = 0 less its strip of width 5 (100 − 5·10), the half wall π·2.5·10"
tex = '2\left(10^2 - \tfrac12\pi 2.5^2\right) + 300 + 50 + \pi\cdot 2.5\cdot 10 = 550 + \tfrac{75}{4}\pi'
+++

A cube of side 10 between the origin and (10, 10, 10), less a cylinder of radius 2.5 and height 10 whose axis is the vertical line through (5, 0), from z = 0 to z = 10: the middle of the side face y = 0, the cylinder's ends in the cube's top and bottom. Half the cylinder, the half with y ≥ 0, is inside the cube. The result is the cube with a half-round notch 5 wide and 2.5 deep in the middle of the face y = 0, running its full height; the cylinder touches no other face.

**As a set:**

$$[0,10]^3 \setminus \{(x-5)^2 + y^2 \le 2.5^2\}$$
