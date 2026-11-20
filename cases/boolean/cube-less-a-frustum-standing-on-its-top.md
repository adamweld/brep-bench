+++
title = "A cube less a frustum standing on its top"
tags = ["boolean", "cut", "analytic", "coincident", "face-contact"]
origin = "OCCT tests/boolean/bopcut_simple/ZF5"

[volume]
value = 216.0
exact = "the cube is unchanged: 6³"
tex = '6^3 = 216'

[area]
value = 216.0
exact = "6·6²"
tex = '6 \cdot 6^2 = 216'
+++

A cube of edge 6 from (−3, −3, 0) to (3, 3, 6), and a cone frustum standing on its top face: axis the z axis, base radius 2 at z = 6, top radius 1 at z = 9. Cut the frustum away from the cube.

The two touch only over the frustum's base disc, which lies wholly inside the cube's top face; nothing overlaps. What is left is the cube, whole.

**As a set:**

$$[-3,3]\times[-3,3]\times[0,6] \setminus \{\rho \le 2 - (z-6)/3,\ 6 \le z \le 9\}$$
