+++
title = "A wedge's slanted face moved out"
tags = ["edit", "move-face", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/H2, H3, H4, H6"

[volume]
value = 456.0
exact = "6·(12.5 + 6.5)/2·8 = 456: the trapezoid x ∈ [0, 12.5] at y = 0, [0, 6.5] at y = 8, extruded 6"
tex = '6\cdot\tfrac{12.5 + 6.5}{2}\cdot 8 = 456'

[area]
value = 374.0
exact = "75 + 39 + 48 + 2·76 + 60: the bottom 12.5·6, the top 6.5·6, the left 8·6, the two trapezoids 76 each, the slanted face 10·6"
tex = '75 + 39 + 48 + 2\cdot 76 + 60 = 374'
+++

A wedge: the polygon in the plane z = 0 with corners (0, 0), (10, 0), (4, 8) and (0, 8), extruded along +z by 6. Its six faces are the bottom, y = 0, 10 by 6; the top, y = 8, 4 by 6; the left side, x = 0, 8 by 6; the two ends, z = 0 and z = 6; and the slanted face from (10, 0) to (4, 8), in the plane 4x + 3y = 40, whose outward normal is (4, 3, 0)/5.

Then edit it directly: move the slanted face 2 outward along its normal, to the plane 4x + 3y = 50. The bottom, the top and the two ends are extended to meet it, so the bottom now runs to x = 12.5 and the top to x = 6.5.

The result is the wedge with the corners (0, 0), (12.5, 0), (6.5, 8) and (0, 8), extruded along +z by 6.

**As a set:**

$$\{\, x \ge 0,\ 0 \le y \le 8,\ 0 \le z \le 6,\ 4x + 3y \le 50 \,\}$$
