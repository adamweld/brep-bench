+++
title = "A block drilled by a cylinder whose ends lie in its top and bottom faces"
tags = ["boolean", "cut", "analytic", "coincident", "hole"]
origin = "OCCT tests/boolean/bopcut_simple/ZP7"

[volume]
value = 2917.2566611769184
exact = "20·16·10 − π·3²·10"
tex = '20\cdot 16\cdot 10 - \pi\cdot 3^2\cdot 10'

[area]
value = 1491.9468914507713
exact = "2(20·16 + 20·10 + 16·10) − 2π·3² + 2π·3·10"
tex = '2(320 + 200 + 160) - 2\pi\cdot 3^2 + 2\pi\cdot 3\cdot 10 = 1360 + 42\pi'
+++

A rectangular block from the origin to (20, 16, 10), less a cylinder of radius 3 and height 10 on the vertical line through (10, 8), from z = 0 to z = 10. The cylinder's end discs lie exactly in the block's bottom and top faces, so it does not run past either. The result is a block with a round hole through it.

**As a set:**

$$[0,20]\times[0,16]\times[0,10] \setminus \{(x-10)^2 + (y-8)^2 \le 3^2\}$$
