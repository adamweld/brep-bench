+++
title = "A B-spline cube and a slab along its diagonal, their common part"
tags = ["nurbs", "bspline", "boolean", "common", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcommon_simple/C8"

[volume]
value = 500
exact = "½·10·10·10"
tex = '\tfrac12\cdot 10\cdot 10\cdot 10'

[area]
value = 441.4213562373095
exact = "2·50 + 10·(10 + 10 + 10√2)"
tex = '2\cdot 50 + 10\,(10 + 10 + 10\sqrt2)'
+++

A B-spline cube and a slab along its diagonal, their common part

A cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it lies flat over the face. And a slab from z = 0 to z = 10, its faces planes, whose footprint is the rectangle with corners (0, 0), (10, 10), (15, 5) and (5, −5): it is 10√2 long, as long as the cube's diagonal, and 5√2 wide, its length along the diagonal of the cube's bottom from the corner (0, 0) to the corner (10, 10), its ends square to the diagonal and through those corners. Make the solid common to both.

The slab's far long side is the plane x − y = 10, which touches the cube only along the vertical edge at (10, 0). The common part is the prism on the right triangle (0, 0), (10, 0), (10, 10), one of its faces the slab's near long side, in the plane x = y.

**As a set:**

$$[0,10]^3 \cap \{\,0 \le x + y \le 20,\ 0 \le x - y \le 10\,\}$$
