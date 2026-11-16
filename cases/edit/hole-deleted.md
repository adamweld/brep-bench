+++
title = "A drilled block's hole deleted: the block is whole again"
tags = ["edit", "delete-face", "cylinder", "planar", "hole", "analytic"]

[volume]
value = 2000.0
exact = "20·10·10 = 2000"
tex = '20\cdot 10\cdot 10 = 2000'

[area]
value = 1000.0
exact = "2(20·10 + 20·10 + 10·10) = 1000"
tex = '2(20\cdot 10 + 20\cdot 10 + 10\cdot 10) = 1000'
+++

A block, the box from the origin to (20, 10, 10), drilled right through along z by a hole of radius
3 whose axis is the vertical line through (10, 5).

Then edit the block directly: delete the hole's wall. Deleting a face takes it out of the solid and
closes the gap it leaves with the faces round it, each run on along its own surface until they meet.
The wall's two ends are the whole of a circle in the top and a circle in the bottom, so those two
faces close over the openings and nothing else changes.

The result is the box from the origin to (20, 10, 10).

**As a set:**

$$[0,20]\times[0,10]\times[0,10]$$
