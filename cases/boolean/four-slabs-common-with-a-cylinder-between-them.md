+++
title = "Four slabs in a grid, their common part with a cylinder standing at their junction"
tags = ["boolean", "common", "cylinder", "multi-body", "analytic"]
origin = "OCCT tests/bugs/modalg_5/bug25614_common"

[volume]
value = 17.45299328594998
exact = "8a, a = (9/2)(arccos(1/3) − arcsin(1/3)) − 2√2 + 1 = 2.18162416: each of the four pieces is the part of the disk of radius 3 in a quadrant x ≥ 1, y ≥ 1 (a = ∫ from x = 1 to 2√2 of (√(9 − x²) − 1) dx), 2 thick"
tex = '8a,\quad a = \tfrac92\left(\arccos\tfrac13 - \arcsin\tfrac13\right) - 2\sqrt2 + 1'

[area]
value = 68.09476747116869
exact = "4(2a + 4(2√2 − 1) + 2ℓ), ℓ = 3(π/2 − 2 arctan(1/(2√2))) = 2.67336752: per piece, its top and bottom a each, its two straight sides, each 2√2 − 1 long and 2 high, and its curved wall, an arc of length ℓ, 2 high. The four pieces do not touch; the area is of all four"
tex = '4\left(2a + 4(2\sqrt2 - 1) + 2\ell\right)'
+++

Four square slabs, each 10 × 10 × 2, in a two-by-two grid with a gap of 2 between them: they fill z from 0 to 2 and, in x and y, the squares [1, 11] or [−11, −1] on each axis. A cylinder of radius 3 stands on the z axis from z = −3 to z = 5, centered in the gap, so that it reaches into a corner of each slab. Take the common part of the slabs and the cylinder. The result is four separate solids, one in each slab's corner, each bounded by two flat sides, the cylinder's wall, and the slab's top and bottom; measure them as one result.

**As a set:**

$$\{1 \le |x| \le 11,\ 1 \le |y| \le 11,\ x^2 + y^2 \le 3^2,\ 0 \le z \le 2\}$$
