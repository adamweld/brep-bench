+++
title = "A block's two rounded top edges deleted: the top and both sides meet again"
tags = ["edit", "delete-face", "fillet", "cylinder", "planar", "analytic"]

[volume]
value = 2000.0
exact = "20·10·10 = 2000"
tex = '20\cdot 10\cdot 10 = 2000'

[area]
value = 1000.0
exact = "2(20·10 + 20·10 + 10·10) = 1000"
tex = '2(20\cdot 10 + 20\cdot 10 + 10\cdot 10) = 1000'
+++

A block, the box from the origin to (20, 10, 10), with the two long edges of its top rounded to
radius 2: the edge where the top, z = 10, meets the side y = 0, and the one where it meets the side
y = 10. Each round is a quarter cylinder: the first about the line y = 2, z = 8, tangent to the top
along y = 2 and to its side along z = 8; the second about the line y = 8, z = 8, tangent to the top
along y = 8 and to its side along z = 8.

Then edit the block directly: delete both rounds together. Deleting a face takes it out of the solid
and closes the gap it leaves with the faces round it, each run on along its own surface until they
meet. The top runs on from y = 2 and from y = 8, and the two sides up from z = 8, until they meet in
the edges y = 0, z = 10 and y = 10, z = 10, and the two ends, x = 0 and x = 20, get their corners
back.

The result is the box from the origin to (20, 10, 10).

**As a set:**

$$[0,20]\times[0,10]\times[0,10]$$
