+++
title = "A recessed plate with a grid of pins"
tags = ["boolean", "cut", "fuse", "analytic", "pattern", "cylinder", "box"]
origin = "OCCT caf/bug31918_2"

[volume]
value = 3224.5486677646163
exact = "40·30·4 − 34·24·2 + 12·π·1²·1.5"
tex = '40\cdot30\cdot4 - 34\cdot24\cdot2 + 12\pi\cdot 1^2\cdot 1.5'

[area]
value = 3305.0973355292326
exact = "1200 + 560 + 384 + 232 + 816 + 12(−π + 3π + π)"
tex = '1200+560+384+232+816+12(-\pi+3\pi+\pi)'
+++

A plate from (0, 0, 0) to (40, 30, 4). In its top face a rectangular recess is cut from the box (3, 3, 2) to (37, 27, 4): it leaves a rim 3 wide all round and a floor at z = 2. On the floor stand twelve upright pins, each a cylinder of radius 1 and height 1.5, from z = 2 to z = 3.5, one on each point of the grid (x, y) with x = 8, 16, 24, 32 and y = 9, 15, 21. The pins are joined to the plate.

**As a set:**

$$\bigl([0,40]\times[0,30]\times[0,4]\bigr)\setminus\bigl([3,37]\times[3,27]\times[2,4]\bigr)\ \cup\ \bigcup_{x,y}\bigl(\{(\xi-x)^2+(\eta-y)^2\le1\}\times[2,3.5]\bigr)$$
