+++
title = "A square block whose faces are inside a post but whose corners poke out, less the post"
tags = ["boolean", "cut", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/boptuc_simple/ZA5"

[volume]
value = 18.102945395836077
exact = "V = 10 (64 − A), A = 48 + 50·(arcsin(4/5) − arcsin(3/5)) the area of the post's disc inside the block's outline"
tex = '10\,(64 - A),\quad A = 48 + 50\bigl(\arcsin\tfrac45 - \arcsin\tfrac35\bigr)'

[area]
value = 140.37941092083278
exact = "area = 2 (64 − A) + 10 (8 + a), where 8 is the block's outline outside the post (each side, 8 long, has a chord 6 long inside the post and 2 outside) and a = 20·(arcsin(4/5) − arcsin(3/5)) is the post's wall inside the outline, four arcs from (4, 3) to (3, 4) and its mirror images"
tex = '2\,(64 - A) + 10\,(8 + a),\quad a = 20\bigl(\arcsin\tfrac45 - \arcsin\tfrac35\bigr)'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −4 to 4, y from −4 to 4 and z from 0 to 10: a square 8 on a side, as tall as the post, its top and bottom faces level with the post's. Cut the post away from the block.

The block's side faces are 4 from the axis, inside the post, whose wall is 5 from it; its four vertical edges, at distance 4√2 ≈ 5.66, poke out through the wall. What is left is the four corners of the block, each a thin piece between the wall and the block's two faces, running to a point at (4, 3) and (3, 4) and their mirror images. The four are separate; build them as one result and measure it whole.

**As a set:**

$$\bigl([-4,4]^2\times[0,10]\bigr) \setminus \{\,\rho \le 5,\ 0 \le z \le 10\,\}$$
