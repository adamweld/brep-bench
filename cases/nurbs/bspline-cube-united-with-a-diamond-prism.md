+++
title = "A cube of flat B-spline faces united with a diamond prism through two of its corners"
tags = ["nurbs", "bspline", "boolean", "union", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bfuse_simple/C9"

[volume]
value = 2500
exact = "10³ + (10√2)²·10 − ½·10³"
tex = '10^3 + (10\sqrt2)^2\cdot 10 - \tfrac12\cdot 10^3'

[area]
value = 1124.2640687119285
exact = "2·250 + 10·(20 + 30√2)"
tex = '2\cdot 250 + 10\,(20 + 30\sqrt2)'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a prism from z = 0 to z = 10 on the square with corners (0, 0), (10, 10), (20, 0) and (10, −10), its faces planes. Unite them.

The square is turned 45° to the cube. Two of its corners, (0, 0) and (10, 10), are corners of the cube's bottom, so its side between them runs along the bottom's diagonal, and its diagonal from (0, 0) to (20, 0) runs along the cube's bottom edge on the x axis and on past it. The two solids share the planes z = 0 and z = 10. They overlap in the prism 10 high on the triangle (0, 0), (10, 0), (10, 10). The union is 10 high on the pentagon (0, 0), (10, −10), (20, 0), (10, 10), (0, 10), and its top and bottom are each one flat face where the operands' faces met.

**As a set:**

$$[0,10]^3 \cup \{\,|x - 10| + |y| \le 10,\ 0 \le z \le 10\,\}$$
