+++
title = "Four slabs united with a post across their joints"
tags = ["boolean", "union", "analytic", "cylinder", "planar", "coincident"]
origin = "OCCT tests/bugs/modalg_5/bug25614_fuse"

[volume]
value = 1252.3893421169302
exact = "V = 20·20·2 + 36π·4 (the plate, and the post's 4 of length outside it: 3 above and 1 below)"
tex = '20\cdot 20\cdot 2 + 36\pi\cdot 4'

[area]
value = 1110.7964473723101
exact = "area = (800 − 72π) + 72π + 160 + 48π: the plate's top and bottom less the post's footprint, the post's two ends, the plate's four sides, and the post's wall outside the plate (2π·6·4)"
tex = '800 + 160 + 48\pi'
+++

Four slabs, each a 10 by 10 by 2 block with z from 0 to 2: the first with x and y from 0 to 10, the second with x from 10 to 20 and y from 0 to 10, the third with x from 0 to 10 and y from 10 to 20, the fourth with x and y from 10 to 20. They touch one another along full faces: the four meet along the vertical line x = 10, y = 10.

A post, the cylinder of radius 6 about that line, from z = −1 to z = 5: it covers the joints between the slabs and runs 1 below the slabs and 3 above them. Make the union of the four slabs and the post, one solid: a plate 20 by 20 by 2 with a post through it.

**As a set:**

$$[0,20]^2\times[0,2] \;\cup\; \{\,(x-10)^2 + (y-10)^2 \le 36,\ -1 \le z \le 5\,\}$$
