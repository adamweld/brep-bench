+++
title = "A wedge's slanted face and top moved in"
tags = ["edit", "move-face", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/H5, H7"

[volume]
value = 257.25
exact = "6·(8.75 + 3.5)/2·7 = 257.25: the trapezoid x ∈ [0, 8.75] at y = 0, [0, 3.5] at y = 7, extruded 6"
tex = '6\cdot\tfrac{8.75 + 3.5}{2}\cdot 7 = 257.25'

[area]
value = 253.75
exact = "52.5 + 21 + 42 + 2·42.875 + 52.5: the bottom 8.75·6, the top 3.5·6, the left 7·6, the two trapezoids 42.875 each, the slanted face 8.75·6"
tex = '52.5 + 21 + 42 + 2\cdot 42.875 + 52.5 = 253.75'
+++

A wedge: the polygon in the plane z = 0 with corners (0, 0), (10, 0), (4, 8) and (0, 8), extruded along +z by 6. Its six faces are the bottom, y = 0, 10 by 6; the top, y = 8, 4 by 6; the left side, x = 0, 8 by 6; the two ends, z = 0 and z = 6; and the slanted face from (10, 0) to (4, 8), in the plane 4x + 3y = 40, whose outward normal is (4, 3, 0)/5.

Then edit it directly: move two adjacent faces in, the top by 1, along −y, to the plane y = 7, and the slanted face by 1 along its inward normal, to the plane 4x + 3y = 35. The faces beside them are trimmed to meet them: the bottom now runs to x = 8.75 and the top to x = 3.5.

The result is the wedge with the corners (0, 0), (8.75, 0), (3.5, 7) and (0, 7), extruded along +z by 6.

**As a set:**

$$\{\, x \ge 0,\ 0 \le y \le 7,\ 0 \le z \le 6,\ 4x + 3y \le 35 \,\}$$
