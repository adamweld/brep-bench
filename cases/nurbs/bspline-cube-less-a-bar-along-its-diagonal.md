+++
title = "A B-spline cube less a bar along its diagonal"
tags = ["nurbs", "bspline", "boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/F5"

[volume]
value = 640.0
exact = "2·(½·8·8)·10"
tex = '2\cdot\left(\tfrac12\cdot 8\cdot 8\right)\cdot 10'

[area]
value = 674.2741699796952
exact = "2·(2·32 + 10·(8 + 8 + 8√2))"
tex = '2\left(2\cdot 32 + 10\,(8 + 8 + 8\sqrt2)\right)'
+++

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a bar from z = 0 to z = 10 whose footprint is the rectangle with corners (1, −1), (11, 9), (9, 11) and (−1, 1), its faces planes: it is 10√2 long, as long as the cube's diagonal, and 2√2 wide, and its axis runs along the diagonal of the cube's bottom from the corner (0, 0) to the corner (10, 10), its two ends square to the diagonal and centred on those corners. Cut the bar from the cube.

Each end of the bar meets the cube only along the vertical edge at the corner it is centred on, and its long sides are the planes y = x + 2 and y = x − 2. The cut takes the strip |x − y| ≤ 2 out of the cube from end to end and top to bottom, and leaves two separate solids, each a prism 10 high on a right triangle with legs 8: one on (0, 2), (0, 10), (8, 10) and one on (2, 0), (10, 0), (10, 8).

**As a set:**

$$[0,10]^3 \setminus \{\,|x - y| \le 2,\ 0 \le x + y \le 20\,\}$$
