+++
title = "A cylinder less a cube that shares its edge, flush at top and bottom"
tags = ["boolean", "cut", "coplanar", "analytic"]
origin = "FreeCAD src/Mod/Part/TestPartApp.py::testMakeCut"

[volume]
value = 376.9911184307752
exact = "¾·π·4²·10 = 120π"
tex = '\tfrac34\pi\cdot 4^2\cdot 10 = 120\pi'

[area]
value = 343.89378290154264
exact = "2·¾π·4² + 2·(4·10) + ¾·2π·4·10 = 80 + 84π"
tex = '2\cdot\tfrac34\pi 4^2 + 2(4\cdot 10) + \tfrac34\cdot 2\pi\cdot 4\cdot 10 = 80 + 84\pi'
+++

A cube of side 10 between the origin and (10, 10, 10), and a cylinder of radius 4 whose axis is the cube's vertical edge along the z axis (x = y = 0), running from z = 0 to z = 10: its two flat ends lie in the planes of the cube's bottom and top. Cut the cube from the cylinder. Three quarters of the cylinder are left, a quarter-sector notched out along the axis: two flat faces meeting at the axis.

**As a set:**

$$\{\,x^2 + y^2 \le 4^2,\ 0 \le z \le 10\,\} \setminus [0,10]^3$$
