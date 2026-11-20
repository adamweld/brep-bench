+++
title = "Four slabs in a grid, less a cylinder standing at their junction"
tags = ["boolean", "cut", "cylinder", "multi-body", "analytic"]
origin = "OCCT tests/bugs/modalg_5/bug25614_cut"

[volume]
value = 782.5470067140500
exact = "4(200 − 2a): each slab 10 × 10 × 2 loses a corner piece of area a, a = ∫ from x = 1 to 2√2 of (√(9 − x²) − 1) dx = (9/2)(arccos(1/3) − arcsin(1/3)) − 2√2 + 1 = 2.18162416, the part of the disk of radius 3 in the quadrant x ≥ 1, y ≥ 1"
tex = '4\,(200 - 2a),\quad a = \tfrac92\left(\arccos\tfrac13 - \arcsin\tfrac13\right) - 2\sqrt2 + 1'

[area]
value = 1094.679112907391
exact = "4(2(100 − a) + 80 − 4(2√2 − 1) + 2ℓ), ℓ = 3(π/2 − 2 arctan(1/(2√2))) = 2.67336752: per slab, its top and bottom less the corner piece, its four sides (80) less the two strips 2(2√2 − 1) the cylinder cuts from the two sides at the corner, and the cylinder's wall in the corner, an arc of length ℓ, 2 high. The four slabs do not touch; the area is of all four"
tex = '4\left(2(100 - a) + 80 - 4(2\sqrt2 - 1) + 2\ell\right)'
+++

Four square slabs, each 10 × 10 × 2, in a two-by-two grid with a gap of 2 between them: they fill z from 0 to 2 and, in x and y, the squares [1, 11] or [−11, −1] on each axis. A cylinder of radius 3 stands on the z axis from z = −3 to z = 5, centered in the gap, so that it bites a rounded corner out of each slab. Cut the cylinder from the slabs. The result is four separate solids, each a slab with one corner rounded off; measure them as one result.

**As a set:**

$$\big(\{1 \le |x| \le 11,\ 1 \le |y| \le 11\} \setminus \{x^2 + y^2 \le 3^2\}\big) \times [0, 2]$$
