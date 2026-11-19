+++
title = "A cube and a block turned with a corner in its face, their common part"
tags = ["boolean", "common", "analytic", "planar", "coincident", "vertex-on-plane"]
origin = "OCCT tests/boolean/bopcommon_simple/C4"

[volume]
value = 93.75
exact = "a prism 10 high on the right triangle with corners (10, 3), (7, 7) and (10, 37/4), whose legs are 5 and 15/4: 10·½·5·15/4 = 375/4"
tex = '10\cdot\tfrac12\cdot 5\cdot\tfrac{15}{4} = \tfrac{375}{4}'

[area]
value = 168.75
exact = "two triangles of 75/8 and three sides 10 high on the triangle's sides 5, 15/4 and 25/4: 75/4 + 150 = 675/4"
tex = '2\cdot\tfrac{75}{8} + 10\left(5 + \tfrac{15}{4} + \tfrac{25}{4}\right) = \tfrac{675}{4}'
+++

A cube from (0, 0, 0) to (10, 10, 10), and a block 10 by 5 by 10 turned about the upright. One
corner of the block is (10, 3, 0), on the cube's face x = 10, and the block's three edges from that
corner run 10 along (4/5, 3/5, 0), 5 along (−3/5, 4/5, 0) and 10 straight up. Make the solid common
to both.

Both stand on z = 0 and are 10 tall, so their tops and their bottoms lie in the same planes. The
block's upright edge through (10, 3, 0) lies in the cube's face x = 10. From that edge the block's
long side runs out of the cube at an angle and its short side into it, to the corner (7, 7, 0), the
one corner of the block inside the cube. The common part is a prism 10 high on the triangle with
corners (10, 3), (7, 7) and (10, 9.25): one of its sides is part of the cube's face x = 10, the
other two parts of the block's sides, and its top and bottom lie in both solids' tops and bottoms.

**As a set:**

$$[0,10]^3 \cap \{\,0 \le 4x + 3y - 49 \le 50,\ 0 \le -3x + 4y + 18 \le 25,\ 0 \le z \le 10\,\}$$
