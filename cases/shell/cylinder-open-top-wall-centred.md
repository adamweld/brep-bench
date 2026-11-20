+++
title = "A cylinder shelled open at the top, its wall centred on its faces, its edges kept sharp"
tags = ["shell", "centred", "cylinder", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestThickness.py::testCurvedOpenShellIsCenteredRadiallyAndAxially"

[volume]
value = 3147.8758388969727
exact = "π(11²·21 − 9²·19) = 1002π: the cylinder of radius 11 from z = −1 to 20 less the cylinder of radius 9 from z = 1 to 20"
tex = '\pi(11^2\cdot 21 - 9^2\cdot 19) = 1002\pi'

[area]
value = 3286.1059156549236
exact = "bottom π·11², outside 2π·11·21, rim π(11² − 9²), inside 2π·9·19, floor π·9²: (121 + 462 + 40 + 342 + 81)π = 1046π"
tex = '(121 + 462 + 40 + 342 + 81)\pi = 1046\pi'
+++

A cylinder of radius 10 on the z axis, from z = 0 to z = 20, shelled to a wall 2 thick, open at its top face, z = 20, with the wall centred on the faces it is grown from: half of it, 1 thick, inside the side and the bottom, and half outside them.

The moved faces are extended until they meet, every edge kept sharp (the outside bottom rim is a sharp circle, not rounded), and run on up to the plane z = 20, where the wall ends in a flat rim, the ring between radii 9 and 11. The side moves to radius 11 outside and 9 inside, the bottom to z = −1 outside and z = 1 inside.

So the solid is the cylinder of radius 11 from z = −1 to z = 20, less the cylinder of radius 9 from z = 1 to z = 20.

**As a set:**

$$\{\rho \le 11,\ -1 \le z \le 20\} \setminus \{\rho < 9,\ 1 < z \le 20\}$$
