+++
title = "A sector of a thick tube, bored, with a cube on its end"
tags = ["boolean", "union", "cut", "cylinder", "planar", "analytic"]
origin = "OCCT boolean/gdml_public/B8"

[volume]
value = 309.7433388230814
exact = "(120/360)π(8² − 4²)·6 − π·1²·6 + 27 = 96π − 6π + 27"
tex = '\tfrac13\pi\left(8^2 - 4^2\right)\cdot 6 - \pi\cdot 6 + 27 = 90\pi + 27'

[area]
value = 366.7433388230814
exact = "the sector 48 (two flat ends) + 32π + 32π (top and bottom) + 32π (outer) + 16π (inner), less the hole's 2π on the top and bottom, plus its wall 12π, plus the cube's six faces 54 less the 9 against the end and 9 of the end"
tex = '48 + 90\pi + \left(54 - 2\cdot 9\right) = 84 + 90\pi'
+++

Part of a thick tube with a hole bored through it and a cube joined to one end.

The tube's section is the annulus between radii 4 and 8 about the z axis, from z = 0 to z = 6, and the part kept is the sector from 0° to 120°, from +x toward +y (right-handed about z). It has two flat ends: one in the plane y = 0 (the rectangle with x from 4 to 8 and z from 0 to 6) and one at 120°.

A hole of radius 1 is bored parallel to the axis, through the whole height, along the line through (3, 3√3), which is the point at radius 6 and 60°. It lies wholly inside the wall.

A cube of side 3 is fused to the end in y = 0: the box from (4.5, −3, 1.5) to (7.5, 0, 4.5), its face y = 0 lying against the end face, wholly inside it.

**As a set:**

$$\{\,4 \le \rho \le 8,\ 0 \le \varphi \le 120^\circ,\ 0 \le z \le 6,\ (x - 3)^2 + (y - 3\sqrt3)^2 \ge 1\,\} \cup [4.5, 7.5] \times [-3, 0] \times [1.5, 4.5]$$
