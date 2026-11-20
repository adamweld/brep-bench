+++
title = "A small cube tilted 45°, one edge dipping below a big cube's top face, less the big cube"
tags = ["boolean", "cut", "analytic", "planar", "turned"]
origin = "OCCT tests/boolean/boptuc_simple/Q9"

[volume]
value = 60.0
exact = "V = 4·(16 − 1): the small cube's section is a diamond of area 16, the part below z = 10 a triangle of area 1, the length 4"
tex = '4\,(16 - 1)'

[area]
value = 90.686291501015240
exact = "area = 2·15 (the end faces, each a diamond less a triangle) + 4·(18 − 2√2) (the four long faces: the section's outline is 16 − 2√2 of the diamond's sides plus the cut's width 2): 102 − 8√2"
tex = '30 + 4\,(18 - 2\sqrt2) = 102 - 8\sqrt2'
+++

A cube with x, y and z from 0 to 10.

A smaller cube of side 4, with two faces square to the x axis, at x = 3 and x = 7, and its other four faces tilted 45° to the horizontal: its section by a plane x = const is a square turned to a diamond, with corners at (y, z) = (5, 9), (5 + 2√2, 9 + 2√2), (5, 9 + 4√2) and (5 − 2√2, 9 + 2√2). Cut the big cube away from the small one.

The small cube's lowest edge, the line y = 5, z = 9 from x = 3 to x = 7, is 1 below the big cube's top face and wholly inside the big cube. The two faces meeting at that edge cross the top face at 45°, along the lines y = 4 and y = 6. What is left is the part of the small cube above z = 10: the cube with a ridge of triangular section, 2 wide at the cut and 1 deep, shaved from its lowest edge.

**As a set:**

$$\bigl([3,7]\times\{\,|y-5| + |z-9-2\sqrt2| \le 2\sqrt2\,\}\bigr) \setminus [0,10]^3$$
