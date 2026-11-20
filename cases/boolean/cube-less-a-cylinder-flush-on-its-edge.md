+++
title = "A cube less a cylinder on one of its edges, flush at top and bottom"
tags = ["boolean", "cut", "coplanar", "analytic"]
origin = "FreeCAD src/Mod/Part/TestPartApp.py::testMakeCut"

[volume]
value = 874.3362938564082
exact = "10³ − ¼·π·4²·10 = 1000 − 40π"
tex = '10^3 - \tfrac14\pi\cdot 4^2\cdot 10 = 1000 - 40\pi'

[area]
value = 557.6991118430775
exact = "600 − 2·4π − 2·(4·10) + ¼·2π·4·10 = 520 + 12π"
tex = '600 - 2\cdot 4\pi - 2(4\cdot 10) + \tfrac14\cdot 2\pi\cdot 4\cdot 10 = 520 + 12\pi'
+++

A cube of side 10 between the origin and (10, 10, 10), and a cylinder of radius 4 whose axis is the cube's vertical edge along the z axis (x = y = 0), running from z = 0 to z = 10: its two flat ends lie in the planes of the cube's bottom and top. Cut the cylinder from the cube. A quarter of the cylinder lies inside the cube, so a quarter-round groove is left running down the edge.

**As a set:**

$$[0,10]^3 \setminus \{\,x^2 + y^2 \le 4^2,\ 0 \le z \le 10\,\}$$
