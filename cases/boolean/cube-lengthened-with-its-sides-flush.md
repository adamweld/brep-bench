+++
title = "A cube lengthened by a block with its sides flush"
tags = ["boolean", "union", "planar", "coincident"]
origin = "OCCT boolean/bopfuse_simple/A2"

[volume]
value = 1500.0
exact = "10·10·15"
tex = '10\cdot 10\cdot 15'

[area]
value = 800.0
exact = "2·10·10 + 4·10·15"
tex = '2\cdot 10\cdot 10 + 4\cdot 10\cdot 15'
+++

A cube of side 10 between the origin and (10, 10, 10), united with a block between (0, 0, 5) and (10, 10, 15). The block has the cube's square section and is slid up by 5, so the two overlap in a block 5 high and all four side faces of one lie in the side planes of the other. The result is one block 15 high.

**As a set:**

$$\bigl([0,10]^2\times[0,10]\bigr) \cup \bigl([0,10]^2\times[5,15]\bigr)$$
