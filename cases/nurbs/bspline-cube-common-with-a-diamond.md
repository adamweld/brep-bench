+++
title = "A cube of flat B-spline faces and a diamond prism, their common part"
tags = ["nurbs", "bspline", "boolean", "common", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bcommon_simple/C9"

[volume]
value = 500.0
exact = "half the cube, ½·10³ = 500"
tex = '\tfrac12\cdot 10^3 = 500'

[area]
value = 441.4213562373095
exact = "two triangles of 50 and three sides 10 high on the triangle's sides 10, 10 and 10√2: 100 + 200 + 100√2 = 300 + 100√2"
tex = '2\cdot 50 + 10\left(10 + 10 + 10\sqrt2\right) = 300 + 100\sqrt2'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the
non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both
ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the
face. And a prism from z = 0 to z = 10 on the square with corners (0, 0), (10, 10), (20, 0) and
(10, −10), its faces planes. Make the solid common to both.

The square is turned 45° to the cube. Two of its corners, (0, 0) and (10, 10), are corners of the
cube's bottom, so its side between them runs along the bottom's diagonal; its diagonal from (0, 0)
to (20, 0) runs along the cube's bottom edge on the x axis and on past it. The two solids share
the planes z = 0 and z = 10. The common part is half the cube, the prism 10 high on the triangle
with corners (0, 0), (10, 0) and (10, 10): its two square sides lie in the cube's faces y = 0 and
x = 10, its triangles in the cube's top and bottom, and its slanting side, through the cube's
diagonal, is the prism's.

**As a set:**

$$[0,10]^3 \cap \{\,|x - 10| + |y| \le 10\,\}$$
