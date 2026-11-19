+++
title = "A stepped block made of two, refined, with a convex and a concave edge chamfered"
tags = ["chamfer", "concave", "refine", "coincident", "planar", "analytic"]
origin = "FreeCAD tests/src/Mod/Part/App/FeatureChamfer.cpp::testOther"

[volume]
value = 41680.0
exact = "(60·10 + 30·15 − ½·5² + ½·3²)·40 = 1042·40 = 41680: the section, the two boxes less the convex chamfer's triangle and with the concave chamfer's added, run 40 along y"
tex = '\left(60\cdot 10 + 30\cdot 15 - \tfrac12\cdot 5^2 + \tfrac12\cdot 3^2\right)\cdot 40 = 41680'

[area]
value = 8696.5483399593904
exact = "2·1042 + 40(60 + 10 + 27 + 3√2 + 12 + 25 + 5√2 + 20) = 8244 + 320√2: the two ends of the section, and its perimeter run 40"
tex = '2\cdot 1042 + 40\left(154 + 8\sqrt2\right) = 8244 + 320\sqrt2'
+++

A stepped block made of two boxes joined, its coplanar faces merged, then two of its edges
chamfered, one convex and one concave.

The boxes: the base from the origin to (60, 40, 10), and on it the upper block from (0, 0, 10) to
(30, 40, 25), flush with the base at x = 0, y = 0 and y = 40. Join them, and merge each pair of
faces lying in one plane into one face: the faces x = 0, y = 0 and y = 40 each become one face, so
the solid's section in a plane y = c is the L through (x, z) = (0, 0), (60, 0), (60, 10), (30, 10),
(30, 25), (0, 25).

Then chamfer, each with two equal distances:

- the convex edge along y at (x, z) = (0, 25), where the merged face x = 0 meets the top z = 25,
  by 5: its face runs from (0, 20) to (5, 25);
- the concave edge along y at (30, 10), where the step's riser x = 30 meets the base's top
  z = 10, by 3: a concave chamfer adds material, filling the corner up to the face from (33, 10) to
  (30, 13).

Each chamfer runs the whole 40 along y and ends on the faces y = 0 and y = 40. The section becomes
the polygon (0, 0), (60, 0), (60, 10), (33, 10), (30, 13), (30, 25), (5, 25), (0, 20), and the solid is
that polygon extruded from y = 0 to y = 40.

**As a set:**

$$\mathrm{poly}\bigl((0,0), (60,0), (60,10), (33,10), (30,13), (30,25), (5,25), (0,20)\bigr) \times [0, 40]$$

with the polygon in (x, z) and the extrusion along y.
