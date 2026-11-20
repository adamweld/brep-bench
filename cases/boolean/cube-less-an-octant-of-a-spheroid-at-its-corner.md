+++
title = "A cube less a prolate spheroid centred on its corner"
tags = ["boolean", "cut", "corner", "ellipsoid", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestTopologicalNamingProblem.py::testPartDesignElementMapSubEllipsoid"

[volume]
value = 976.4380550980766
exact = "1000 − ⅛·(4/3)π·3²·5"
tex = '1000 - \tfrac18\cdot\tfrac43\pi\cdot 3^2\cdot 5'

[area]
value = 590.0936043695585
exact = "600 − quarter ellipses ¼π·3² + 2·¼π·3·5, plus the octant's surface ⅛·2π·3²(1 + (c/(a e))·asin e), e = 4/5: = 600 − 15π/2 + (75π/16)·asin(4/5)"
tex = '600 - \tfrac94\pi - \tfrac{15}{2}\pi + \tfrac94\pi\bigl(1 + \tfrac{25}{12}\arcsin\tfrac45\bigr)'
+++

The cube from the origin to (10, 10, 10), less the ellipsoid centred on the origin, the cube's corner, with its axis of revolution along z: semi-axes 3 along x, 3 along y and 5 along z. An octant of it lies in the cube, with its three flat sides in the cube's faces x = 0, y = 0 and z = 0.

**As a set:**

$$K \setminus \{\,(x^2 + y^2)/9 + z^2/25 \le 1\,\}, \quad K = [0,10]^3$$
