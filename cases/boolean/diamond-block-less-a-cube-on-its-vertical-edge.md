+++
title = "A block turned to a diamond on a cube's vertical edge, less the cube"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "turned"]
origin = "OCCT tests/boolean/boptuc_simple/C3"

[volume]
value = 1500
exact = "10·(200 − 50): the diamond's area less the triangle the cube covers, 10 high"
tex = '10\,(200 - 50)'

[area]
value = 924.2640687119285
exact = "2·150 for the two ends, plus 10·(30√2 + 20) for the walls: three diamond sides of 10√2 and two cube sides of 10"
tex = '2\cdot 150 + 10\,(30\sqrt2 + 20)'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 10 high whose footprint is a square turned a quarter of a right angle, a diamond with its corners at (20, 10), (10, 20), (0, 10) and (10, 0), standing on z = 0. Its edges run along the diagonal directions (1, 1, 0) and (1, −1, 0), 10√2 long. The diamond's centre (10, 10) is the cube's vertical edge from (10, 10, 0) to (10, 10, 10), and two of its corners, (0, 10) and (10, 0), are corners of the cube's top and bottom faces. Cut the cube away from the block.

The cube covers the triangle (0, 10), (10, 10), (10, 0), half of the diamond's half. What is left is a prism 10 high on the polygon with corners (10, 0), (20, 10), (10, 20), (0, 10), (10, 10), standing on z = 0.

**As a set:**

$$\{\,|x-10| + |y-10| \le 10,\ 0 \le z \le 10\,\} \setminus [0,10]^3$$
