+++
title = "A cube and a turned block over its corner and touching two more, their common part"
tags = ["boolean", "common", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/bopcommon_simple/D4"

[volume]
value = 625
exact = "a prism 10 high on the trapezoid with corners (0, 0), (10, 0), (5/2, 10), (0, 10), whose parallel sides are 10 and 5/2 and whose height is 10: 10·½·(10 + 5/2)·10"
tex = '10\cdot\tfrac12\left(10 + \tfrac52\right)\cdot 10'

[area]
value = 475
exact = "two trapezoids of 125/2 and four sides 10 high on the trapezoid's sides 10, 25/2, 5/2 and 10: 125 + 10·35"
tex = '2\cdot\tfrac{125}{2} + 10\left(10 + \tfrac{25}{2} + \tfrac52 + 10\right)'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 12 by 15 by 10 turned about the upright. One corner of the block is (1, -8, 0), and the block's three edges from that corner run 12 along (4/5, 3/5, 0), 15 along (−3/5, 4/5, 0) and 10 straight up. Make the solid common to both.

Both stand on z = 0 and are 10 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (1, −8), (53/5, −4/5), (8/5, 56/5) and (−8, 4), all outside the cube. The cube's corner (0, 0) is inside the block; its corner (10, 10) is outside; its corners (10, 0) and (0, 10) lie exactly on the block's short end side and long side, so two of the block's upright faces pass through two upright edges of the cube.

The common part is a prism 10 high on the trapezoid with corners (0, 0), (10, 0), (5/2, 10) and (0, 10): two of its sides lie in the cube's faces x = 0 and y = 0, the slanted ones in the block's two sides.

**As a set:**

$$[0,10]^3 \cap \{\,0 \le 4x + 3y + 20 \le 60,\ 0 \le -3x + 4y + 35 \le 75,\ 0 \le z \le 10\,\}$$
