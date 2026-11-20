+++
title = "Two overlapping blocks joined to their mirror image across a plane through the overlap"
tags = ["pattern", "mirror", "union", "analytic", "planar", "coincident"]
origin = "FreeCAD tests/src/Mod/Part/App/FeatureMirroring.cpp::testYMirrorWithExistingElementMap"

[volume]
value = 2300
exact = "10·230: the section is 14·5 + 16·5 + 16·5 = 230"
tex = '10\cdot 230'

[area]
value = 1080
exact = "2·230 + 62·10: the two ends and the outline's perimeter 14 + 5 + 1 + 10 + 16 + 10 + 1 + 5 = 62 run 10"
tex = '2\cdot 230 + 10\cdot 62'
+++

Two blocks 10 high, both standing on the xy plane: the first from (0, 0, 0) to (10, 10, 10), the second from (5, 5, 0) to (15, 15, 10). Unite them, and reflect the result in the plane x = 7, which cuts through their overlap. Join the reflection to the original.

The reflection is the two blocks from (4, 0, 0) to (14, 10, 10) and from (−1, 5, 0) to (9, 15, 10). The whole is symmetric about x = 7: one solid, 10 high, with the outline (0, 0), (14, 0), (14, 5), (15, 5), (15, 15), (−1, 15), (−1, 5), (0, 5). The tops lie in one plane and so do the bottoms.

**As a set:**

$$S \cup \sigma(S),\quad S = \bigl([0,10]^2 \cup [5,15]^2\bigr)\times[0,10],\quad \sigma(x,y,z) = (14 - x, y, z)$$
