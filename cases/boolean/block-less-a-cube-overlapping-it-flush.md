+++
title = "A block less a cube it overlaps, their side faces flush"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/boptuc_simple/A2"

[volume]
value = 500
exact = "5·10·10: the block's half beyond the cube"
tex = '5\cdot 10\cdot 10'

[area]
value = 400
exact = "2·(5·10 + 5·10 + 10·10): the result is a block 5 by 10 by 10"
tex = '2\,(5\cdot 10 + 5\cdot 10 + 10\cdot 10)'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block from (5, 0, 0) to (15, 10, 10). The block has the cube's cross-section in y and z and is slid 5 along x, so the two overlap in the block from x = 5 to x = 10 and their faces y = 0, y = 10, z = 0 and z = 10 lie in common planes. Cut the cube away from the block.

What is left is the block from (10, 0, 0) to (15, 10, 10).

**As a set:**

$$[5,15]\times[0,10]^2 \setminus [0,10]^3$$
