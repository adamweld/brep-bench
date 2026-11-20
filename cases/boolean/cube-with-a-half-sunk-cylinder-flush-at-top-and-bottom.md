+++
title = "A cube with a cylinder half sunk into one side, flush with its top and bottom"
tags = ["boolean", "union", "analytic", "cylinder", "planar", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/ZN8"

[volume]
value = 1062.8318530717959
exact = "1000 + (π·2²/2)·10: the cube and the half of the cylinder outside it"
tex = '1000 + \tfrac{\pi\cdot 2^2}{2}\cdot 10'

[area]
value = 635.398223686155
exact = "560 + 2·(2π) + 2π·2·10/2: the cube's 600 less the 4 × 10 strip of face the cylinder covers, plus the two half discs level with the top and bottom and the half wall outside the cube"
tex = '600 - 40 + 2\cdot 2\pi + 20\pi'
+++

A cube with x, y and z from 0 to 10.

A cylinder of radius 2, its axis the vertical line through (5, 0), the middle of the cube's face y = 0, from z = 0 to z = 10: as tall as the cube, its ends level with the cube's top and bottom faces. Make the union of the two.

Half the cylinder lies inside the cube. The other half stands out of the face y = 0, a half-round bulge on it; the cylinder's wall meets the face square, along the vertical lines x = 3 and x = 7, y = 0.

**As a set:**

$$[0,10]^3 \cup \{\,(x-5)^2 + y^2 \le 2^2,\ 0 \le z \le 10\,\}$$
