+++
title = "A cube of flat B-spline faces and a block sharing three of its face planes, their common part"
tags = ["nurbs", "bspline", "boolean", "common", "analytic", "planar", "coincident"]
origin = "OCCT tests/bugs/moddata_2/bug408"

[volume]
value = 560
exact = "10 × 7 × 8"
tex = '10\cdot 7\cdot 8'

[area]
value = 412
exact = "2(10·7 + 10·8 + 7·8)"
tex = '2\left(70 + 80 + 56\right)'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the
non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both
ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the
face. And a block from (0, 0, 0) to (15, 7, 8), its faces planes. Make the solid common to both.

The block's faces x = 0, y = 0 and z = 0 lie in the cube's faces; its faces y = 7 and z = 8 cut
through the cube, and its face x = 15 lies outside it. The common part is the box from (0, 0, 0) to
(10, 7, 8): three of its faces are the cube's, one of them the whole of the cube's face x = 10 cut
to the block's width, and three are the block's.

**As a set:**

$$[0,10]^3 \cap \bigl([0,15] \times [0,7] \times [0,8]\bigr)$$
