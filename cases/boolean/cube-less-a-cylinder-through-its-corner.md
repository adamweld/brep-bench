+++
title = "A cube less a cylinder standing on its corner, through its bottom"
tags = ["boolean", "cut", "corner", "cylinder", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestTopologicalNamingProblem.py::testPartDesignElementMapSubCylinder"

[volume]
value = 924.601776313845
exact = "1000 − ¼·π·4²·6: the cylinder meets the cube in a quarter of itself, 6 high"
tex = '1000 - \tfrac14\pi\cdot 4^2\cdot 6'

[area]
value = 589.6991118430775
exact = "600 − two sides 2·(4·6) − the bottom's quarter disc 4π + the quarter cylinder's wall ½π·4·6 + its quarter-disc end inside the cube 4π = 552 + 12π"
tex = '600 - 48 - 4\pi + 12\pi + 4\pi'
+++

The cube from the origin to (10, 10, 10), less the cylinder of radius 4 about the z axis from z = −3 to z = 6. The axis runs along the cube's vertical edge at the origin, so a quarter of the cylinder lies in the cube (the other three quarters lie outside it): it opens through the bottom, and its two flat sides lie in the cube's faces x = 0 and y = 0. Its top, z = 6, is inside the cube.

**As a set:**

$$K \setminus \{\,x^2 + y^2 \le 16,\ -3 \le z \le 6\,\}, \quad K = [0,10]^3$$
