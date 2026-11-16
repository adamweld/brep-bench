+++
title = "A half-round notch in a block's side deleted: the side closes over it"
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

A block, the box from the origin to (20, 10, 10), with a half-round notch in its side y = 0: the
cylinder of radius 3 about the vertical line through (10, 0) is cut away, top to bottom. The notch's
wall is half a cylinder, its two straight edges both in the side y = 0, at x = 7 and x = 13.

Then edit the block directly: delete the notch's wall. Deleting a face takes it out of the solid and
closes the gap it leaves with the faces round it, each run on along its own surface until they meet.
The wall's edges are not a whole loop in any one face: they run through the side, the top and the
bottom. The side runs on from x = 7 and from x = 13 until it is one rectangle again, and the top and
the bottom close in over the half disks the notch took from them.

The result is the box from the origin to (20, 10, 10).

**As a set:**

$$[0,20]\times[0,10]\times[0,10]$$
