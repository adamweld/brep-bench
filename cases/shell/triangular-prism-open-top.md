+++
title = "A triangular prism lofted between two triangles, shelled with its top open"
tags = ["shell", "inward", "loft", "planar", "analytic"]
origin = "OCCT tests/bugs/modalg_3/bug5805_55"

[volume]
value = 186
exact = "240 − 6·9: the prism, 24·10, less the cavity, a triangle of area 6 (the base triangle scaled by a half about its incenter) over a height of 9"
tex = '24\cdot 10 - 6\cdot 9'

[area]
value = 396
exact = "24 + 24·10 + (24 − 6) + 6 + 12·9: the bottom, the sides, the rim at z = 10, the cavity's floor and its sides"
tex = '24 + 240 + (24 - 6) + 6 + 108'
+++

A triangular prism made by a ruled loft: its three sides ruled between the triangle with corners
(0, 0, 0), (6, 0, 0), (0, 8, 0) and the equal triangle with corners (0, 0, 10), (6, 0, 10),
(0, 8, 10), joined corner to corner, and closed by the two triangles' faces. The base is a right
triangle with sides 6, 8 and 10.

Shell it inward to a wall 1 thick, open at its top face, z = 10. Each face moves 1 inward, and
the moved faces run on until they meet the plane of the opening and one another, so the cavity's
corners stay sharp. The base triangle's inscribed circle has radius 2, so the cavity's triangle
is the base triangle scaled by one half about its incenter (2, 2): corners (1, 1), (4, 1), (1, 5),
sides 3, 4 and 5. The cavity runs from the floor at z = 1 up to the opening, and the rim at the
top is the ring of triangle between the two.

The result is the prism less that cavity: a cup with a triangular bore.

**As a set:**

$$\bigl(T \times [0, 10]\bigr) \setminus \bigl((T \ominus B_1) \times [1, 10]\bigr), \quad T = \operatorname{conv}\{(0,0), (6,0), (0,8)\}$$
