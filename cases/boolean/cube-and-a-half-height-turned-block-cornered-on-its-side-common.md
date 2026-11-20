+++
title = "A cube and a block half as tall turned with a corner on its face, their common part"
tags = ["boolean", "common", "analytic", "planar", "coincident", "vertex-on-plane"]
origin = "OCCT tests/boolean/bopcommon_simple/L5"

[volume]
value = 62.5
exact = "a prism 5 high on the quadrilateral with corners (4, 8), (5/2, 10), (0, 10), (0, 5), of area 25/2: 5·25/2"
tex = '5\cdot\tfrac{25}{2}'

[area]
value = 100
exact = "two quadrilaterals of 25/2 and four sides 5 high on the quadrilateral's sides 5/2, 5/2, 5 and 5: 25 + 5·15"
tex = '2\cdot\tfrac{25}{2} + 5\left(\tfrac52 + \tfrac52 + 5 + 5\right)'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 5 by 4 by 5 turned about the upright. One corner of the block is (0, 5, 0), and the block's three edges from that corner run 5 along (4/5, 3/5, 0), 4 along (−3/5, 4/5, 0) and 5 straight up. Make the solid common to both.

The block stands on z = 0 like the cube but is only 5 tall, so its bottom lies in the cube's bottom and its top is inside the cube. On the xy plane the block's corners are (0, 5), (4, 8), (8/5, 56/5) and (−12/5, 41/5). The corner (0, 5) lies on the cube's face x = 0 and (4, 8) is inside the cube; the other two are outside it, and no cube corner is strictly inside the block, though the cube's corner (0, 10) lies on the block's long side.

The common part is a prism 5 high on the quadrilateral with corners (4, 8), (5/2, 10), (0, 10) and (0, 5), standing on z = 0.

**As a set:**

$$[0,10]^3 \cap \{\,0 \le 4x + 3y - 15 \le 25,\ 0 \le -3x + 4y - 20 \le 20,\ 0 \le z \le 5\,\}$$
