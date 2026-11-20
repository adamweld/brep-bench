+++
title = "A cube united with a wide cylinder beside it, cutting into one face"
tags = ["boolean", "union", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopfuse_simple/ZO6"

[volume]
value = 2007.074072087854
exact = "1000 + 360π − 10·S, S = 36α − 8√5 the circular segment x ≤ 10 of the cylinder's section, α = arccos(2/3)"
tex = '1000 + 360\pi - 10\,S,\quad S = 36\alpha - 8\sqrt5,\ \alpha = \arccos\tfrac23'

[area]
value = 988.0349732802027
exact = "2·(100 + 36π − S) + 300 + (100 − 40√5) + 60·(2π − 2α): the top and bottom, the cube's faces x = 0, y = 0 and y = 10, its face x = 10 less the chord strip 2√20·10, and the cylinder's wall outside the cube"
tex = '2\,(100 + 36\pi - S) + 300 + (100 - 40\sqrt5) + 60\,(2\pi - 2\alpha)'
+++

A cube with x, y and z from 0 to 10.

A cylinder of radius 6, its axis the vertical line through (14, 5), from z = 0 to z = 10: as tall as the cube, its ends level with the cube's top and bottom faces. Its axis is 4 beyond the cube's face x = 10, so its wall cuts across the face and its body enters the cube to x = 8. Make the union of the two.

The cylinder's wall meets the face x = 10 along the two vertical lines y = 5 ± √20, well inside the face, and nowhere touches the cube's other faces.

**As a set:**

$$[0,10]^3 \cup \{\,(x-14)^2 + (y-5)^2 \le 6^2,\ 0 \le z \le 10\,\}$$
