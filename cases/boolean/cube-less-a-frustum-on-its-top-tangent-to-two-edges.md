+++
title = "A cube less a frustum on its top, tangent to two top edges"
tags = ["boolean", "cut", "analytic", "tangent", "coincident", "face-contact"]
origin = "OCCT tests/boolean/bopcut_simple/ZG8"

[volume]
value = 216.0
exact = "the cube is unchanged: 6³"
tex = '6^3 = 216'

[area]
value = 216.0
exact = "6·6²"
tex = '6 \cdot 6^2 = 216'
+++

A cube of edge 6 from (0, 0, 0) to (6, 6, 6), and a cone frustum standing on its top face: axis the vertical through (2, 2), base radius 2 at z = 6, top radius 1 at z = 9. Cut the frustum away from the cube.

The frustum's base circle, centred at (2, 2, 6), is tangent to the top face's edges x = 0 (at (0, 2, 6)) and y = 0 (at (2, 0, 6)). The two touch only over the base disc, which lies in the top face; nothing overlaps. What is left is the cube, whole.

**As a set:**

$$[0,6]^3 \setminus \{(x-2)^2 + (y-2)^2 \le 2 - (z-6)/3,\ 6 \le z \le 9\}$$
