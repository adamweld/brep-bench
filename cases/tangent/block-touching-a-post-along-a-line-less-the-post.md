+++
title = "A block touching a post along a line, less the post"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar", "tangent-line"]
origin = "OCCT tests/boolean/boptuc_simple/U7"

[volume]
value = 320.0
exact = "V = 4·8·10: the block, untouched"
tex = '4 \cdot 8 \cdot 10'

[area]
value = 304.0
exact = "area = 2·(4·8) + 10·(2·4 + 2·8): the block's surface"
tex = '2\cdot 32 + 10\cdot 24'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from 5 to 9, y from −4 to 4 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Cut the post away from the block.

The block's face x = 5 touches the post's wall along the line x = 5, y = 0, from z = 0 to z = 10, where block and post are tangent. They have no volume in common, and the cut leaves the block as it was.

**As a set:**

$$\bigl([5,9]\times[-4,4]\times[0,10]\bigr) \setminus \{\,\rho \le 5,\ 0 \le z \le 10\,\}$$
