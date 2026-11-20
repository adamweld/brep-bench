+++
title = "A square block with one corner on a post's wall, less the post"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/boptuc_simple/S2"

[volume]
value = 69.562361400839452
exact = "V = 10 (9 − A), A = (25/2)·arcsin(3/5) − 6 the area of the post's disc inside the block's outline"
tex = '10\,(9 - A),\quad A = \tfrac{25}{2}\arcsin\tfrac35 - 6'

[area]
value = 126.08752771983211
exact = "area = 2 (9 − A) + 10 (8 + a), where 8 is the block's outline outside the post (12 less its faces y = 4, 3 long, and x = 0 from y = 4 to y = 5, 1 long, which lie inside the post) and a = 5·arcsin(3/5) the post's wall inside the outline, from (3, 4) to (0, 5)"
tex = '2\,(9 - A) + 10\,(8 + a),\quad a = 5 \arcsin\tfrac35'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from 0 to 3, y from 4 to 7 and z from 0 to 10: its section is a square of side 3, and it is as tall as the post, its top and bottom faces level with the post's. Cut the post away from the block.

The block's corner at (3, 4) lies exactly on the post's wall, 5 from the axis. The block's corner (0, 4) is inside the post, (0, 7) and (3, 7) outside it. So the post covers the part of the block below the wall's arc, which runs from (3, 4) to (0, 5): the block's face y = 4 is inside the post, and its face x = 3 starts on the wall and goes out. What is left is the rest of the block, above the arc, in one piece.

**As a set:**

$$\bigl([0,3]\times[4,7]\times[0,10]\bigr) \setminus \{\,\rho \le 5,\ 0 \le z \le 10\,\}$$
