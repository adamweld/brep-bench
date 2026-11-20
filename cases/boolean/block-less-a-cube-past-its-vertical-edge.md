+++
title = "A block centred on a cube's vertical edge, less the cube"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT tests/boolean/boptuc_simple/J9"

[volume]
value = 750
exact = "10·(10² − 5²): the block's section less the quarter the cube covers, 10 high"
tex = '10\,(10^2 - 5^2)'

[area]
value = 550
exact = "2·75 for the two ends, plus 10·40 for the walls round the L, whose boundary is 40 long"
tex = '2\cdot 75 + 10\cdot 40'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block from (5, 5, 0) to (15, 15, 10). The block is centred on the cube's vertical edge from (10, 10, 0) to (10, 10, 10), covers a quarter of its own section with the cube, and is as tall as the cube, so its top and bottom lie in the cube's top and bottom planes. Cut the cube away from the block.

What is left is a prism 10 high on the L-shaped polygon with corners (10, 5), (15, 5), (15, 15), (5, 15), (5, 10), (10, 10), standing on z = 0.

**As a set:**

$$[5,15]^2\times[0,10] \setminus [0,10]^3$$
