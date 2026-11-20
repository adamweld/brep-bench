+++
title = "A block turned 30°, its corners crossing a cube's sides, within the cube's height, less the cube"
tags = ["boolean", "cut", "analytic", "planar", "turned"]
origin = "OCCT tests/boolean/boptuc_simple/Q7"

[volume]
value = 3.9793808954285138
exact = "V = 4 wedges, each a triangle of area 14√3 − 24 (legs meeting at a right angle, hypotenuse on the cube's side) over height 4: 16·(14√3 − 24)"
tex = '16\,(14\sqrt3 - 24)'

[area]
value = 42.564064605510183
exact = "area = 4·2·(14√3 − 24) (wedge tops and bottoms) + 4·4·(2√3 − 2) (wedge sides, two legs of total 2√3 − 2 each, height 4) + 4·4·(8 − 4√3) (the cube's sides where the wedges were, hypotenuse 8 − 4√3): 80√3 − 96"
tex = '8\,(14\sqrt3 - 24) + 16\,(2\sqrt3 - 2) + 16\,(8 - 4\sqrt3) = 80\sqrt3 - 96'
+++

A cube with x, y and z from 0 to 10.

A block whose section is a square of side 8: the square [−4, 4]² about the point (5, 5), turned 30° counterclockwise seen from above about the vertical axis through (5, 5). It runs from z = 3 to z = 7, inside the cube's height. Cut the cube away from the block.

Each of the block's four vertical edges, at distance 4√2 ≈ 5.66 from the axis, pokes 2√3 − 3 ≈ 0.46 out through one of the cube's side faces, at a slant. The block's top and bottom faces are inside the cube. What is left is four small wedges, one at each corner of the block, each a right-angled triangle in plan, 4 tall, standing against the cube's side; they touch nothing of each other. Build the four as one result and measure it whole.

**As a set:**

$$\bigl(Q\times[3,7]\bigr) \setminus [0,10]^3,\quad Q = \text{the square } [-4,4]^2 \text{ about } (5,5), \text{ turned } 30^\circ$$
