+++
title = "A cube and a slab along its diagonal whose far side reaches a corner, their common part"
tags = ["boolean", "common", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcommon_simple/F9, OCCT boolean/bcommon_simple/G1, OCCT boolean/bcommon_simple/G2, OCCT boolean/bcommon_simple/G3"

[volume]
value = 500
exact = "½·10·10·10"
tex = '\tfrac12\cdot 10\cdot 10\cdot 10'

[area]
value = 441.4213562373095
exact = "2·50 + 10·(10 + 10 + 10√2)"
tex = '2\cdot 50 + 10\,(10 + 10 + 10\sqrt2)'
+++

A cube and a slab along its diagonal whose far side reaches a corner, their common part

A cube from (0, 0, 0) to (10, 10, 10) and a slab from z = 0 to z = 10 whose footprint is the rectangle with corners (0, 0), (10, 10), (15, 5) and (5, −5): it is 10√2 long, as long as the cube's diagonal, and 5√2 wide, one long side along the diagonal of the cube's bottom from the corner (0, 0) to the corner (10, 10), its ends square to the diagonal and through those corners. Make the part common to both.

The slab's other long side is the plane x − y = 10, which touches the cube only along the vertical edge at (10, 0). The common part is the prism on the right triangle (0, 0), (10, 0), (10, 10).

**As a set:**

$$[0,10]^3 \cap \{\,0 \le x + y \le 20,\ 0 \le x - y \le 10\,\}$$
