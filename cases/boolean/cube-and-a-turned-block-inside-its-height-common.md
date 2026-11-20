+++
title = "A cube and a turned block lying within its height, their common part"
tags = ["boolean", "common", "analytic", "planar", "coincident", "vertex-on-plane"]
origin = "OCCT tests/boolean/bopcommon_simple/Q7"

[volume]
value = 83.33333333333333
exact = "a prism 4 high on the pentagon with corners (1, 0), (5, 3), (2, 7), (0, 11/2), (0, 4/3), of area 125/6: 4·125/6"
tex = '4\cdot\tfrac{125}{6}'

[area]
value = 115
exact = "two pentagons of 125/6 and five sides 4 high on the pentagon's sides 5, 5, 5/2, 25/6 and 5/3, which total 55/3: 125/3 + 220/3"
tex = '2\cdot\tfrac{125}{6} + 4\left(5 + 5 + \tfrac52 + \tfrac{25}{6} + \tfrac53\right)'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 5 by 5 by 4 turned about the upright and standing from z = 3 to z = 7. One corner of the block is (1, 0, 3), and the block's three edges from that corner run 5 along (4/5, 3/5, 0), 5 along (−3/5, 4/5, 0) and 4 straight up. Make the solid common to both.

The block lies between the cube's bottom and top faces, so neither of its end faces touches a face of the cube. On the xy plane the block's corners are (1, 0), (5, 3), (2, 7) and (−2, 4). The corner (1, 0) lies on the cube's face y = 0, the corners (5, 3) and (2, 7) are inside the cube and (−2, 4) is outside it; none of the cube's corners is inside the block.

The common part is a prism 4 high, from z = 3 to z = 7, on the pentagon with corners (1, 0), (5, 3), (2, 7), (0, 11/2) and (0, 4/3): its sides run along the block's long side from (1, 0) to (5, 3), its short side from (5, 3) to (2, 7), its other long side from (2, 7) out to the cube's face x = 0, along that face from (0, 11/2) down to (0, 4/3), and along the block's first short side back to (1, 0).

**As a set:**

$$[0,10]^3 \cap \{\,0 \le 4x + 3y - 4 \le 25,\ 0 \le -3x + 4y + 3 \le 25,\ 3 \le z \le 7\,\}$$
