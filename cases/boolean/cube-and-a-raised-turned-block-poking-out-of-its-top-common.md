+++
title = "A cube and a raised turned block poking out of its top, their common part"
tags = ["boolean", "common", "analytic", "planar", "general-position"]
origin = "OCCT tests/boolean/bopcommon_simple/P2"

[volume]
value = 199.79166666666666
exact = "a prism 5 high on the pentagon with corners (5, 1), (10, 19/4), (10, 10), (26/3, 10), (2, 5), of area 959/24: 5·959/24"
tex = '5\cdot\tfrac{959}{24}'

[area]
value = 210.75
exact = "two pentagons of 959/24 and five sides 5 high on the pentagon's sides 25/4, 21/4, 4/3, 25/3 and 5, which total 157/6: 959/12 + 5·157/6"
tex = '2\cdot\tfrac{959}{24} + 5\left(\tfrac{25}{4} + \tfrac{21}{4} + \tfrac43 + \tfrac{25}{3} + 5\right)'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 10 by 5 by 10 turned about the upright and raised so that it stands from z = 5 to z = 15. One corner of the block is (5, 1, 5), and the block's three edges from that corner run 10 along (4/5, 3/5, 0), 5 along (−3/5, 4/5, 0) and 10 straight up. Make the solid common to both.

On the xy plane the block's corners are (5, 1), (13, 7), (10, 11) and (2, 5). Two of them, (5, 1) and (2, 5), are inside the cube's outline and the other two are outside it; the cube's corner (10, 10) is inside the block's outline and its other three corners are outside it. No corner of either solid lies on the other's boundary. The block's bottom face is inside the cube, at z = 5, and its top face is above the cube, so the cube's top face z = 10 cuts the block across.

The common part is a prism standing on z = 5 and ending in the cube's top face z = 10, 5 high, on the pentagon with corners (5, 1), (10, 19/4), (10, 10), (26/3, 10) and (2, 5). Its sides lie in the block's long side from (5, 1), the cube's face x = 10, the cube's face y = 10, the block's other long side, and the block's short side back to (5, 1).

**As a set:**

$$[0,10]^3 \cap \{\,0 \le 4x + 3y - 23 \le 50,\ 0 \le -3x + 4y + 11 \le 25,\ 5 \le z \le 15\,\}$$
