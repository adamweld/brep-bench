+++
title = "A cube united with a small cube tilted 45°, one edge dipping below the big cube's top face"
tags = ["boolean", "union", "analytic", "planar", "turned"]
origin = "OCCT tests/boolean/bopfuse_simple/Q9"

[volume]
value = 1060.0
exact = "V = 1000 + 64 − 4·1 (the small cube's overlap with the cube is a prism of length 4 and a triangular section of area 1)"
tex = '1000 + 64 - 4\cdot 1'

[area]
value = 674.68629150101524
exact = "area = 600 + 96 − 8 − 8√2 − 2: the two cubes, less the big top face's strip (width 2, length 4), the two tilted faces' strips below z = 10 (√2 × 4 each) and the small cube's two end triangles (1 each)"
tex = '600 + 96 - 8 - 8\sqrt2 - 2'
+++

A cube with x, y and z from 0 to 10.

A smaller cube of side 4, with two faces square to the x axis, at x = 3 and x = 7, and its other four faces tilted 45° to the horizontal: its section by a plane x = const is a square turned to a diamond, with corners at (y, z) = (5, 9), (5 + 2√2, 9 + 2√2), (5, 9 + 4√2) and (5 − 2√2, 9 + 2√2). Make the union of the two cubes.

The small cube's lowest edge, the line y = 5, z = 9 from x = 3 to x = 7, is 1 below the big cube's top face and wholly inside the big cube. The two faces meeting at that edge cross the top face at 45°, along the lines y = 4 and y = 6, and the rest of the small cube stands above.

**As a set:**

$$\bigl([0,10]^3\bigr) \cup \bigl([3,7]\times\{\,|y-5| + |z-9-2\sqrt2| \le 2\sqrt2\,\}\bigr)$$
