+++
title = "A cube's corner notched by a block raised above it"
tags = ["boolean", "cut", "analytic", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/P3"

[volume]
value = 875.0
exact = "1000 − 5³: the block takes the cube's corner cube [5, 10]³"
tex = '1000 - 5^3'

[area]
value = 600.0
exact = "600: the notch removes three faces of 25 and adds three of 25"
tex = '600'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block from (5, 5, 5) to (15, 15, 15), whose lower corner is inside the cube and whose upper part stands clear above the cube's top face and beside its sides. Cut the block away from the cube. No face of the block lies in a face of the cube. The cube loses the small cube from (5, 5, 5) to (10, 10, 10) at its top corner.

**As a set:**

$$[0,10]^3 \setminus [5,15]^3$$
