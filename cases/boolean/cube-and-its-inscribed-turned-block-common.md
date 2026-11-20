+++
title = "A cube and a turned block with its four corners on the cube's sides, their common part"
tags = ["boolean", "common", "analytic", "planar", "coincident", "vertex-on-plane"]
origin = "OCCT tests/boolean/bopcommon_simple/F5"

[volume]
value = 175
exact = "the block itself: 5·5·7"
tex = '5\cdot 5\cdot 7'

[area]
value = 190
exact = "the block's two 5 by 5 ends and four 5 by 7 sides: 2·25 + 4·35"
tex = '2\cdot 25 + 4\cdot 5\cdot 7'
+++

A cube from (0, 0, 0) to (7, 7, 7), and a block 5 by 5 by 7 turned about the upright. One corner of the block is (3, 0, 0), and the block's three edges from that corner run 5 along (4/5, 3/5, 0), 5 along (−3/5, 4/5, 0) and 7 straight up. Make the solid common to both.

Both stand on z = 0 and are 7 tall, so their tops lie in one plane and their bottoms in another. On the xy plane the block's corners are (3, 0), (7, 3), (4, 7) and (0, 4), one on each side of the cube's square, none at a cube corner. The whole block is inside the cube, its four upright edges touching the cube's four faces, so the common part is the block.

**As a set:**

$$[0,7]^3 \cap \{\,0 \le 4x + 3y - 12 \le 25,\ 0 \le -3x + 4y + 9 \le 25,\ 0 \le z \le 7\,\}$$
