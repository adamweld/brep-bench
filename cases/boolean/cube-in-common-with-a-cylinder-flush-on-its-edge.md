+++
title = "A cube and a cylinder on one of its edges, flush at top and bottom, their common part"
tags = ["boolean", "common", "coplanar", "analytic"]
origin = "FreeCAD src/Mod/Part/TestPartApp.py::testMakeCommon"

[volume]
value = 125.66370614359172
exact = "¼·π·4²·10 = 40π"
tex = '\tfrac14\pi\cdot 4^2\cdot 10 = 40\pi'

[area]
value = 167.96459430051422
exact = "2·¼π·4² + 2·(4·10) + ¼·2π·4·10 = 80 + 28π"
tex = '2\cdot\tfrac14\pi 4^2 + 2(4\cdot 10) + \tfrac14\cdot 2\pi\cdot 4\cdot 10 = 80 + 28\pi'
+++

A cube of side 10 between the origin and (10, 10, 10), and a cylinder of radius 4 whose axis is the cube's vertical edge along the z axis (x = y = 0), running from z = 0 to z = 10: its two flat ends lie in the planes of the cube's bottom and top. Make the part common to both: a quarter of the cylinder, a sector 90° wide whose two flat sides lie in the cube's faces x = 0 and y = 0.

**As a set:**

$$[0,10]^3 \cap \{\,x^2 + y^2 \le 4^2,\ 0 \le z \le 10\,\}$$
