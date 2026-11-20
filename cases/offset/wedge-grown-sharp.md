+++
title = "A wedge grown by 1, its edges kept sharp"
tags = ["offset", "grow", "sharp-edge", "planar", "oblique", "analytic"]
origin = "OCCT tests/offset/shape_type_i/E8"

[volume]
value = 740.0
exact = "the polygon through (−1, −1), (12, −1), (4.5, 9), (−1, 9), of area 92.5, times 8"
tex = '92.5 \cdot 8'

[area]
value = 513.0
exact = "2·92.5 + 41·8: the two ends and the sides, the polygon's perimeter being 13 + 12.5 + 5.5 + 10 = 41"
tex = '2 \cdot 92.5 + 41 \cdot 8'
+++

The wedge with its base the rectangle from (0, 0, 0) to (10, 0, 6) in the plane y = 0, and its far
face the smaller rectangle from (0, 8, 0) to (4, 8, 6) in the plane y = 8. It has six flat faces:
the base, the far face, the bottom z = 0, the top z = 6, the side x = 0, and a slanted face through
the edges from (10, 0, 0) to (4, 8, 0) and from (10, 0, 6) to (4, 8, 6), which is the plane 8x + 6y
= 80.

Grow it by 1 with its edges kept sharp: move each face 1 outward along its normal, and extend the
moved faces until they meet, so that no round is added and every edge stays where its two faces
meet.

The moved planes are y = −1, y = 9, z = −1, z = 7, x = −1 and 8x + 6y = 90. The grown wedge is the
solid between them, with the same six faces: the plane 8x + 6y = 90 meets y = −1 at x = 12 and y = 9
at x = 4.5, so its base is the rectangle from (−1, −1, −1) to (12, −1, 7) and its far face the one
from (−1, 9, −1) to (4.5, 9, 7).

**As a set:**

$$\{\, x \ge -1,\ -1 \le y \le 9,\ -1 \le z \le 7,\ 8x + 6y \le 90 \,\}$$
