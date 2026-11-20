+++
title = "A row of cylinders braced by slabs"
tags = ["boolean", "union", "cylinder", "planar", "analytic"]
origin = "OCCT boolean/gdml_public/B5"

[volume]
value = 358.0688720283045
exact = "108π + 8g, g = 10 − 2√3 − 4π/3: three cylinders of radius 2 and height 9, and in each of the eight places a slab crosses between two cylinders, the strip |y| ≤ 1 between their sides, area g, 1 thick"
tex = '108\pi + 8g,\quad g = 10 - 2\sqrt3 - \tfrac{4\pi}{3}'

[area]
value = 443.30801367457116
exact = "the cylinders' sides 108π and ends 24π, and at each of the eight bridges its top and bottom 2g, its two straight sides 2(5 − 2√3), less the 4π/3 of the two cylinders' sides it covers: 132π + 8(2g + 2(5 − 2√3) − 4π/3)"
tex = '132\pi + 8\left(2g + 2\left(5 - 2\sqrt3\right) - \tfrac{4\pi}{3}\right)'
+++

Three cylinders in a row, braced by four slabs, all fused.

The cylinders have radius 2, stand from z = 0 to z = 9 and are centered on the line y = 0 at x = 0, 5 and 10; they do not touch (their sides are 1 apart). Each slab is a stadium of thickness 1: the rectangle from x = 0 to x = 10 and y from −1 to 1, with a half disk of radius 1 on each end (centered at x = 0 and x = 10), extruded 1 along z. The four slabs lie at z from 1 to 2, 3 to 4, 5 to 6 and 7 to 8, each through the middle of all three cylinders, so that each bridges the two gaps between them. Each slab's ends lie inside the end cylinders.

**As a set**, with a stadium T = [0, 10] × [−1, 1] ∪ {(x, y): x² + y² ≤ 1 or (x − 10)² + y² ≤ 1}:

$$\bigcup_{c \in \{0,5,10\}} \{\,(x - c)^2 + y^2 \le 2^2,\ 0 \le z \le 9\,\} \ \cup\ \bigcup_{k=0}^{3} T \times [2k + 1,\, 2k + 2]$$
