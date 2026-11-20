+++
title = "A cube united with a cylinder on one of its edges, flush at top and bottom"
tags = ["boolean", "union", "coplanar", "analytic"]
origin = "FreeCAD src/Mod/Part/TestPartApp.py::testMakeFuse"

[volume]
value = 1376.9911184307753
exact = "10³ + ¾·π·4²·10 = 1000 + 120π"
tex = '10^3 + \tfrac34\pi\cdot 4^2\cdot 10 = 1000 + 120\pi'

[area]
value = 783.8937829015426
exact = "2(100 + 12π) + (400 − 80) + ¾·2π·4·10 = 520 + 84π"
tex = '2(100 + 12\pi) + (400 - 80) + \tfrac34\cdot 2\pi\cdot 4\cdot 10 = 520 + 84\pi'
+++

A cube of side 10 between the origin and (10, 10, 10), and a cylinder of radius 4 whose axis is the cube's vertical edge along the z axis (x = y = 0), running from z = 0 to z = 10: its two flat ends lie in the planes of the cube's bottom and top. Unite them. A quarter of the cylinder lies inside the cube; the other three quarters stand outside, beside the faces x = 0 and y = 0.

**As a set:**

$$[0,10]^3 \cup \{\,x^2 + y^2 \le 4^2,\ 0 \le z \le 10\,\}$$
