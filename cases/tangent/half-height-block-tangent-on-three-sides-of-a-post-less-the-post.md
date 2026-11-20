+++
title = "A block half as tall as a post, tangent on three sides and cutting across it, less the post"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar", "tangent-line"]
origin = "OCCT tests/boolean/boptuc_simple/V7"

[volume]
value = 63.212820551477374
exact = "V = 5 (80 − A), A = 25π − (25·arccos(3/5) − 12) the area of the disc of radius 5 inside the block's outline"
tex = '5\,(80 - A),\quad A = 25\pi - \bigl(25\arccos\tfrac35 - 12\bigr)'

[area]
value = 276.0
exact = "area = 2 (80 − A) + 5 (28 + a), where 28 = 36 − 8 is the block's outline outside the post and a = 10π − 10·arccos(3/5) is the length of the post's circle inside the outline"
tex = '2\,(80 - A) + 5\,(28 + a),\quad a = 10\pi - 10\arccos\tfrac35'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10.

A block with x from −5 to 3, y from −5 to 5 and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base, its top inside the post's height. Cut the post away from the block.

The block's faces x = −5, y = −5 and y = 5 each touch the post's wall along a line, where block and post are tangent. Its face x = 3 cuts across the post, along a chord 8 long. What is left is the block less its part inside the post: four pieces: in y ≥ 0 a corner piece at (−5, 5) and another at (3, 5) above the arc, and their mirror images in y ≤ 0 at (−5, −5) and (3, −5). The pieces at (−5, ±5) touch the pieces at (3, ±5) along the lines where the post's wall meets the faces y = ±5, at (0, ±5), and the two at (−5, 5) and (−5, −5) touch each other along the line at (−5, 0). Build it all as one result, and measure it whole.

**As a set:**

$$\bigl([-5,3]\times[-5,5]\times[0,5]\bigr) \setminus \{\,\rho \le 5,\ 0 \le z \le 10\,\}$$
