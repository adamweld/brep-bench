+++
title = "A cube less a ball centred on its corner"
tags = ["boolean", "cut", "corner", "sphere", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestTopologicalNamingProblem.py::testPartDesignElementMapSubSphere"

[volume]
value = 966.4896783617089
exact = "1000 − ⅛·(4/3)π·4³"
tex = '1000 - \tfrac18\cdot\tfrac43\pi\cdot 4^3'

[area]
value = 587.4336293856409
exact = "600 − three quarter discs 3·¼π·4² + the octant's sphere ⅛·4π·4² = 600 − 12π + 8π"
tex = '600 - 3\cdot\tfrac14\pi\cdot 4^2 + \tfrac18\cdot 4\pi\cdot 4^2'
+++

The cube from the origin to (10, 10, 10), less the ball of radius 4 centred on the origin, the cube's corner. An octant of it lies in the cube, with its three flat sides in the cube's faces x = 0, y = 0 and z = 0.

**As a set:**

$$K \setminus \{\,x^2 + y^2 + z^2 \le 16\,\}, \quad K = [0,10]^3$$
