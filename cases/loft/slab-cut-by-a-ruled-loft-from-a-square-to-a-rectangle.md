+++
title = "A slab cut by a ruled loft from a square to a rectangle"
tags = ["loft", "ruled", "cut", "planar", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestLoft.py::testSimpleSubtractiveLoftCase"

[volume]
value = 543.36
exact = "588 − 4∫₀³ (4 − u²) dz, u = (z + 1)/5: the slab 14·14·3 less the hole, 44.64"
tex = '196\cdot 3 - 4\int_0^3 \left(4 - u^2\right)dz,\quad u = \tfrac{z + 1}{5}'

[area]
value = 579.6705873304908
exact = "slab 2·14² + 4·14·3 = 560, less the hole's openings 4(4 − 0.2²) at z = 0 and 4(4 − 0.8²) at z = 3, plus its four slanted walls (a + b = 4 at every height, slope 1/5): 48·√(1 + 1/25) = 48√26/5"
tex = '560 - 15.84 - 13.44 + 48\sqrt{1 + \tfrac{1}{25}}'
+++

A square slab cut through by a ruled loft from a square to a rectangle, leaving a hole whose four walls are flat and slanted.

The slab is the box −7 ≤ x, y ≤ 7, 0 ≤ z ≤ 3. The tool is a ruled loft between two horizontal rectangles centered on the z axis: the square −2 ≤ x, y ≤ 2 at z = −1, and the rectangle −1 ≤ x ≤ 1, −3 ≤ y ≤ 3 at z = 4. Corners are joined to the corners at the same side of both, (±2, ±2, −1) to (±1, ±3, 4) with the same signs, by straight lines, and each side of the tool is the ruled surface between the corresponding edges: the edge of the square at x = 2 (parallel to y) and the edge of the rectangle at x = 1 (also parallel to y) bound a flat quadrilateral, and likewise for the other three sides. So at height z the tool's section is the rectangle |x| ≤ 2 − u, |y| ≤ 2 + u, u = (z + 1)/5. The tool runs past the slab at both faces; the slab less the tool is the solid, a slab with a slanted hole through it, 3.6 × 4.4 at the slab's bottom face and 2.4 × 5.6 at its top, narrowing in x and widening in y as it rises.

**As a set:**

$$\{|x|, |y| \le 7,\ 0 \le z \le 3\} \setminus \{\,|x| < 2 - u,\ |y| < 2 + u\,\},\quad u = \tfrac{z + 1}{5}$$
