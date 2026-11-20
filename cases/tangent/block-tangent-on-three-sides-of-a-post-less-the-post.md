+++
title = "A block tangent to a post on three sides and cutting across it, less the post"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar", "tangent-line"]
origin = "OCCT tests/boolean/boptuc_simple/R3"

[volume]
value = 126.42564110295475
exact = "V = 10 (80 − A), A = 25π − (25·arccos(3/5) − 12) the area of the disc of radius 5 inside the block's outline (the disc less its segment beyond x = 3)"
tex = '10\,(80 - A),\quad A = 25\pi - \bigl(25\arccos\tfrac35 - 12\bigr)'

[area]
value = 526.71487177940905
exact = "area = 2 (80 − A) + 10 (28 + a), where 28 = 36 − 8 is the block's outline outside the post (its face x = 3 loses its chord of 8 to the post) and a = 10π − 10·arccos(3/5) is the length of the post's circle inside the outline"
tex = '2\,(80 - A) + 10\,(28 + a),\quad a = 10\pi - 10\arccos\tfrac35'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −5 to 3, y from −5 to 5 and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Cut the post away from the block.

The block's faces x = −5, y = −5 and y = 5 each touch the post's wall along a line, where block and post are tangent. Its face x = 3 cuts across the post, along a chord 8 long from y = −4 to y = 4. Its corners at (3, ±5) are outside the post.

What is left is the block less its part inside the post: two pieces, one in y ≥ 0 and one in y ≤ 0, that touch along the line where the post's wall meets the face x = −5, at (−5, 0). The part left can be in several pieces, even pieces that touch only along a line; build it all as one result, and measure it whole.

**As a set:**

$$\bigl([-5,3]\times[-5,5]\times[0,10]\bigr) \setminus \{\,\rho \le 5,\ 0 \le z \le 10\,\}$$
