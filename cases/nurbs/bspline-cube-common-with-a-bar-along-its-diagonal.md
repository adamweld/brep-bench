+++
title = "A B-spline cube and a bar along its diagonal, their common part"
tags = ["nurbs", "bspline", "boolean", "common", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcommon_simple/D1"

[volume]
value = 360
exact = "(100 − 2·32)·10"
tex = '(100 - 2\cdot 32)\cdot 10'

[area]
value = 378.2741699796952
exact = "2·36 + 10·(8 + 16√2)"
tex = '2\cdot 36 + 10\,(8 + 16\sqrt2)'
+++

A B-spline cube and a bar along its diagonal, their common part

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a bar from z = 0 to z = 10 whose footprint is the rectangle with corners (1, −1), (11, 9), (9, 11) and (−1, 1), its faces planes: it is 10√2 long, as long as the cube's diagonal, and 2√2 wide, its axis along the diagonal of the cube's bottom from the corner (0, 0) to the corner (10, 10), its two ends square to the diagonal and centred on those corners. Make the solid common to both.

Each end of the bar meets the cube only along the vertical edge at the corner it is centred on, and its long sides are the planes y = x + 2 and y = x − 2. The common part is the strip |x − y| ≤ 2 of the cube, 10 high: a prism on the hexagon (0, 0), (2, 0), (10, 8), (10, 10), (8, 10), (0, 2).

**As a set:**

$$[0,10]^3 \cap \{\,|x - y| \le 2,\ 0 \le x + y \le 20\,\}$$
