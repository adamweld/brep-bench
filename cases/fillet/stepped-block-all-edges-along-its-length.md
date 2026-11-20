+++
title = "A stepped block with all eight of its edges along its length rounded"
tags = ["fillet", "concave", "planar", "cylinder", "analytic", "refine"]
origin = "FreeCAD tests/src/Mod/Part/App/FeatureFillet.cpp::testMostEdges"

[volume]
value = 18518.230016469244
exact = "(625 − 36 + 9π)·30: the section (625) less (1 − π/4)·9 at each of its six convex corners and plus as much at each of the two concave, run 30"
tex = '\left(589 + 9\pi\right)\cdot 30 = 17670 + 270\pi'

[area]
value = 4825.522023056942
exact = "2(589 + 9π) + 30(82 + 12π) = 3638 + 378π: the two ends and the section's outline, 130 less 8·(2 − π/2)·3, run 30"
tex = '2\left(589 + 9\pi\right) + 30\left(82 + 12\pi\right) = 3638 + 378\pi'
+++

A stepped block made of two boxes joined, its coplanar faces merged, then rounded on every edge that runs along y. The boxes: the base from the origin to (40, 30, 10), and on it the upper block from (10, 0, 10) to (25, 30, 25), flush with the base at y = 0 and y = 30. Join them, and merge the two faces in the plane y = 0 into one face, and the two in y = 30 into one. The solid's section in a plane y = c is the polygon through (x, z) = (0, 0), (40, 0), (40, 10), (25, 10), (25, 25), (10, 25), (10, 10), (0, 10).

The eight edges of the solid along y run through those eight points: six convex, two concave (at (25, 10) and (10, 10)). Round all eight with radius 3. A convex round is a quarter cylinder cut from the corner; a concave round is a quarter cylinder's worth of material filled into it. Each round runs the whole 30 along y and ends on the faces y = 0 and y = 30; the edges round the two ends are not rounded. The section's corners become quarter circles of radius 3 and the solid is that section extruded from y = 0 to y = 30.

**As a set:**

The 2D region of the section in (x, z), rounded so that its six convex corners and two concave corners are quarter circles of radius 3 (centers inside at (3, 3), (37, 3), (37, 7), (22, 22), (13, 22), (3, 7); outside at (28, 13), (7, 13)), extruded from y = 0 to y = 30.
