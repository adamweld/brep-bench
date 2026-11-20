+++
title = "An oblique pyramid lofted from a square to a point"
tags = ["loft", "ruled", "planar", "vertex", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestPipe.py::testMultisectionPipeSketchVertexSection"

[volume]
value = 48
exact = "A·h/3 = 36·4/3"
tex = '\tfrac{1}{3}\cdot 36\cdot 4'

[area]
value = 96.77394122695176
exact = "36 + 3(√32 + √32 + √20 + √20): the base and four triangles on base edges 6 long, whose apex is √32 from the lines y = −3 and x = −3 (4 across and 4 up) and √20 from y = 3 and x = 3 (2 across and 4 up): 36 + 24√2 + 12√5"
tex = '36 + 3\left(2\sqrt{32} + 2\sqrt{20}\right) = 36 + 24\sqrt{2} + 12\sqrt{5}'
+++

A solid lofted between a square and a single point, by flat ruled faces: the square of side 6 in the
plane z = 0, from (−3, −3, 0) to (3, 3, 0), and the point (1, 1, 4), which is not above its centre
but above the point (1, 1) of it. The section at the point is a point, a vertex,
and not a face: each side of the square is joined to the point by a flat triangle.

It is an oblique pyramid on a square, its apex 4 up and to one side.

**As a set:**

$$\operatorname{conv}\bigl([-3,3]^2 \times \{0\} \,\cup\, \{(1, 1, 4)\}\bigr)$$
