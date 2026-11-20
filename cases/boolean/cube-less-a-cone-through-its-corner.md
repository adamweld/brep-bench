+++
title = "A cube less a cone standing on its corner, through its bottom"
tags = ["boolean", "cut", "corner", "cone", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestTopologicalNamingProblem.py::testPartDesignElementMapSubCone"

[volume]
value = 966.4896783617089
exact = "1000 − ¼·⅓π·4²·8: the cone's part above z = 0 is a cone of base radius 4, height 8, and a quarter of it is in the cube"
tex = '1000 - \tfrac14\cdot\tfrac13\pi\cdot 4^2\cdot 8'

[area]
value = 583.5328883098038
exact = "600 − two sides 2·(½·4·8) − the bottom's quarter disc 4π + the quarter lateral ¼·π·4·√80 = 568 − 4π + 4√5·π"
tex = '600 - 32 - 4\pi + 4\sqrt5\,\pi'
+++

The cube from the origin to (10, 10, 10), less the cone with its axis on the z axis, its base the disc of radius 5 at z = −2 and its apex at (0, 0, 8): at height z its radius is (8 − z)/2. A quarter of it lies in the cube, with its two flat sides in the cube's faces x = 0 and y = 0; it opens through the bottom as a quarter disc of radius 4 and runs up to the apex at z = 8, inside the cube.

**As a set:**

$$K \setminus \{\,\sqrt{x^2 + y^2} \le (8 - z)/2,\ -2 \le z \le 8\,\}, \quad K = [0,10]^3$$
