+++
title = "A wedge shrunk by 1"
tags = ["offset", "shrink", "planar", "oblique", "analytic"]
origin = "OCCT tests/offset/shape_type_i/E9"

[volume]
value = 114.0
exact = "the polygon through (1, 1), (8, 1), (3.5, 7), (1, 7), of area 28.5, times 4"
tex = '28.5 \cdot 4'

[area]
value = 149.0
exact = "2·28.5 + 23·4: the two ends and the sides, the polygon's perimeter being 7 + 7.5 + 2.5 + 6 = 23"
tex = '2 \cdot 28.5 + 23 \cdot 4'
+++

The wedge with its base the rectangle from (0, 0, 0) to (10, 0, 6) in the plane y = 0, and its far
face the smaller rectangle from (0, 8, 0) to (4, 8, 6) in the plane y = 8. It has six flat faces:
the base, the far face, the bottom z = 0, the top z = 6, the side x = 0, and a slanted face through
the edges from (10, 0, 0) to (4, 8, 0) and from (10, 0, 6) to (4, 8, 6), which is the plane 8x + 6y
= 80.

Shrink it by 1: the solid of every point at least 1 from the wedge's outside.

The moved planes are y = 1, y = 7, z = 1, z = 5, x = 1 and 8x + 6y = 70. The shrunk wedge is the
solid between them, with the same six faces: the plane 8x + 6y = 70 meets y = 1 at x = 8 and y = 7
at x = 3.5, so its base is the rectangle from (1, 1, 1) to (8, 1, 5) and its far face the one from
(1, 7, 1) to (3.5, 7, 5).

**As a set:**

$$\{\, x \ge 1,\ 1 \le y \le 7,\ 1 \le z \le 5,\ 8x + 6y \le 70 \,\}$$
