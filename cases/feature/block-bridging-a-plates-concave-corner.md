+++
title = "A block with a rounded corner bridging a plate's concave corner"
tags = ["feature", "rib", "linear", "union", "planar", "cylinder", "analytic"]
origin = "OCCT tests/feat/featlf/D5"

[volume]
value = 11141.502089981976
exact = "10·|P| + 10·|B∖P| + 6·|B∩P|, with the plate's footprint |P| = 864 (shoelace), |B∩P| = ∫₁₀¹⁸ (18 − 4x/9) dx + ∫₁₈²⁸ (2 + 4(40 − x)/11) dx (the block's footprint below the plate's two sides there) and |B∖P| = 324 − (4 − π)·4 − |B∩P| (the rounded corner lies outside the plate)"

[area]
value = 3761.697466590311
exact = "|P| (bottom) + |B| (the block's top, 324 − (4 − π)·4) + (|P| − |B∩P|) (the plate's top less the block's foot) + |B∖P| (the block's underside) + 10·(the plate's perimeter outside the block) + 6·(its two sides inside the block, √(64 + (32/9)²) + 10√(1 + 16/121), under the block's start at z = 6) + 10·(the block's perimeter outside the plate, including 2π for the rounded corner) + 6·(its perimeter inside the plate)"
+++

A plate 10 thick with a concave corner: the pentagon with corners (0, 0), (40, 0), (40, 14), (18, 22)
and (0, 30) in the xy plane, from z = 0 to z = 10. Its sides from (40, 14) to (18, 22) and from
(18, 22) to (0, 30) meet at the concave corner (18, 22).

A block bridges that corner. Its footprint is the rectangle 10 ≤ x ≤ 28, 12 ≤ y ≤ 30 with its corner
at (28, 30) rounded by a quarter circle of radius 4 about (24, 26), and it runs from z = 6, 4 below
the plate's top, up to z = 16, 6 above it. It covers the corner and fills out beyond the two sides,
where its underside at z = 6 shows; its rounded corner lies wholly outside the plate. The plate and
the block are one solid.

**As a set:**

$$\bigl(\mathrm{poly}\bigl((0, 0), (40, 0), (40, 14), (18, 22), (0, 30)\bigr) \times [0, 10]\bigr) \cup \bigl(B \times [6, 16]\bigr),\quad B = [10, 28] \times [12, 30] \setminus \{\,x > 24,\ y > 26,\ (x - 24)^2 + (y - 26)^2 > 16\,\}$$
