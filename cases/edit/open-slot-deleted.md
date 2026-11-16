+++
title = "An open slot in a block's side deleted: the side closes over it"
tags = ["edit", "delete-face", "cylinder", "planar", "analytic"]

[volume]
value = 2000.0
exact = "20·10·10 = 2000"
tex = '20\cdot 10\cdot 10 = 2000'

[area]
value = 1000.0
exact = "2(20·10 + 20·10 + 10·10) = 1000"
tex = '2(20\cdot 10 + 20\cdot 10 + 10\cdot 10) = 1000'
+++

A block, the box from the origin to (20, 10, 10), with a slot cut into its side y = 0, top to
bottom: 4 wide, from x = 8 to x = 12, running in to y = 5 and closed there by a half cylinder of
radius 2 about the vertical line through (10, 5). The slot has three faces: the flats x = 8 and x =
12, and the half cylinder, tangent to both.

Then edit the block directly: delete all three. Deleting a face takes it out of the solid and closes
the gap it leaves with the faces round it, each run on along its own surface until they meet. The
side y = 0 runs on across the slot's mouth, and the top and the bottom close in over the slot's
outline.

The result is the box from the origin to (20, 10, 10).

**As a set:**

$$[0,20]\times[0,10]\times[0,10]$$
