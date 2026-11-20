+++
title = "A cube less a torus centred on its corner"
tags = ["boolean", "cut", "corner", "torus", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestTopologicalNamingProblem.py::testPartDesignElementMapSubTorus"

[volume]
value = 950.6519779945532
exact = "1000 − ⅛·2π²Rr², R = 5, r = 2: ¼π²·5·4"
tex = '1000 - \tfrac18\cdot 2\pi^2 R r^2,\ R = 5,\ r = 2'

[area]
value = 605.3657248551897
exact = "600 − the bottom's quarter ring π·R·r = 10π − two half discs 2·½πr² = 4π + the eighth of the torus's surface ⅛·4π²Rr = 5π²"
tex = '600 - \pi R r - \pi r^2 + \tfrac12\pi^2 R r'
+++

The cube from the origin to (10, 10, 10), less the torus about the z axis with its centre at the origin (the cube's corner): its tube's circle has radius 2 and its centre runs round the circle of radius 5 in the plane z = 0. An eighth of it lies in the cube, the part above the plane z = 0 of the quarter round the axis. Its flat sides are two half discs in the cube's faces x = 0 and y = 0 and a quarter ring in its bottom z = 0.

**As a set:**

$$K \setminus \{\,(\sqrt{x^2 + y^2} - 5)^2 + z^2 \le 4\,\}, \quad K = [0,10]^3$$
