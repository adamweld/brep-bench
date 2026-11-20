+++
title = "A cylinder in common with the two pieces a wider cylinder cuts it into"
tags = ["boolean", "common", "compound-tool", "shared-faces", "analytic"]
origin = "FreeCAD tests/src/Mod/Part/App/TopoShapeExpansion.cpp::makeElementBooleanCommonWithCompoundTool"

[volume]
value = 75.39822368615503
exact = "πr²h, r = 2, h = 6: = 24π (the two pieces together are the cylinder)"
tex = '\pi r^2 h = 24\pi,\quad r = 2,\ h = 6'

[area]
value = 100.53096491487338
exact = "2πr² + 2πrh = 8π + 24π = 32π"
tex = '2\pi r^2 + 2\pi r h = 32\pi'
+++

Let A be the cylinder of radius 2 on the z axis, from z = 0 to z = 6, and B the cylinder of radius 3
with its axis the vertical line through (1.5, 0), from z = 0 to z = 3. B is wider and shorter than A
and off its axis: it takes the part of A below z = 3 that lies within 3 of its axis and leaves the
rest (the point (−2, 0, 0) of A's floor is 3.5 from B's axis, outside it).

Cut A into two solids: P, the part of A inside B, and Q, the rest of A, the part outside B together
with the whole of A above z = 3. P and Q touch along the wall of B inside A and along the plane
z = 3 where P's top meets Q.

Take the common part of A with the tool made of P and Q held together as one compound shape. Since
the two pieces together are all of A, the result is A, one cylinder.

**As a set:**

$$A \cap (P \cup Q) = A, \qquad P = A \cap B,\quad Q = A \setminus B$$
