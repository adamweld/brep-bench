+++
title = "A stepped block with all eight of its edges along its length chamfered"
tags = ["chamfer", "concave", "planar", "analytic", "refine"]
origin = "FreeCAD tests/src/Mod/Part/App/FeatureChamfer.cpp::testMost"

[volume]
value = 18210
exact = "(625 − 4·4.5)·30: the section (40·10 + 15·15 = 625) less a corner triangle of 4.5 at each of its six convex corners and plus one at each of the two concave, run 30"
tex = '\left(625 - 4\cdot\tfrac{9}{2}\right)\cdot 30 = 18210'

[area]
value = 4692.233764908628
exact = "2·607 + 30·(82 + 24√2) = 3674 + 720√2: the two ends and the section's outline, 130 less 8·6 plus 8·3√2, run 30"
tex = '2\cdot 607 + 30\left(82 + 24\sqrt2\right) = 3674 + 720\sqrt2'
+++

A stepped block made of two boxes joined, its coplanar faces merged, then chamfered on every edge that runs along y. The boxes: the base from the origin to (40, 30, 10), and on it the upper block from (10, 0, 10) to (25, 30, 25), flush with the base at y = 0 and y = 30. Join them, and merge the two faces in the plane y = 0 into one face, and the two in y = 30 into one. The solid's section in a plane y = c is the polygon through (x, z) = (0, 0), (40, 0), (40, 10), (25, 10), (25, 25), (10, 25), (10, 10), (0, 10).

The eight edges of the solid along y run through those eight points: six convex, two concave (at (25, 10) and (10, 10)). Chamfer all eight, each with two equal distances of 3. A convex chamfer cuts the corner's triangle away; a concave one fills it. Each chamfer runs the whole 30 along y and ends on the faces y = 0 and y = 30; the edges round the two ends are not chamfered. The section becomes a polygon of 16 vertices, and the solid is that polygon extruded from y = 0 to y = 30.

**As a set:**

The section polygon in (x, z) through (3, 0), (37, 0), (40, 3), (40, 7), (37, 10), (28, 10), (25, 13), (25, 22), (22, 25), (13, 25), (10, 22), (10, 13), (7, 10), (3, 10), (0, 7), (0, 3), extruded from y = 0 to y = 30.
