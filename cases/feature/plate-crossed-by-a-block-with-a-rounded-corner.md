+++
title = "A plate crossed by a sunk block with a rounded corner"
tags = ["feature", "rib", "linear", "union", "planar", "cylinder", "analytic"]
origin = "OCCT tests/feat/featlf/D1"

[volume]
value = 10110.826795114726
exact = "10·|P| + 10·|B∖P| + 6·|B∩P|, with the plate's footprint |P| = 878 (shoelace), the block's footprint |B∖P| = ∫₈¹⁸ (4 + 3y/14) dy = 67.857… beyond the plate's side x = 40 − 3y/14, and |B∩P| = ∫₈¹⁸ (14 − 3y/14) dy − (4 − π)·4 = 112.142857… − (4 − π)·4 (the rounded corner lies inside the plate)"

[area]
value = 3486.695441493361
exact = "|P| (bottom) + |B| (the block's top, 180 − (4 − π)·4) + (|P| − |B∩P|) (the plate's top less the block's foot) + |B∖P| (the block's underside) + 10·(the plate's perimeter outside the block) + 6·(its perimeter inside the block, under the block's start at z = 6) + 10·(the block's perimeter outside the plate) + 6·(its perimeter inside the plate, above the plate's top); the part of the plate's side x = 40 − 3y/14 inside the block runs 10√(1 + (3/14)²), and the block's rounded corner contributes 2π in place of 8"
+++

A plate 10 thick: the quadrilateral with corners (0, 0), (40, 0), (34, 28) and (4, 22) in the xy
plane, from z = 0 to z = 10.

A block crosses its side from (40, 0) to (34, 28). Its footprint is the rectangle 26 ≤ x ≤ 44,
8 ≤ y ≤ 18 with its corner at (26, 18) rounded by a quarter circle of radius 4 about (30, 14), and it
runs from z = 6, 4 below the plate's top, up to z = 16, 6 above it. So it lies partly inside the
plate and partly out beyond the side, where its underside at z = 6 shows. The plate and the block
are one solid.

**As a set:**

$$\bigl(\mathrm{poly}\bigl((0, 0), (40, 0), (34, 28), (4, 22)\bigr) \times [0, 10]\bigr) \cup \bigl(B \times [6, 16]\bigr),\quad B = [26, 44] \times [8, 18] \setminus \{\,x < 30,\ y > 14,\ (x - 30)^2 + (y - 14)^2 > 16\,\}$$
