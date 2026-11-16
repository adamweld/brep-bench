+++
title = "An L-block's step deleted: the block is whole again"
tags = ["edit", "delete-face", "planar", "concave-edge", "analytic"]

[volume]
value = 2400.0
exact = "20·15·8 = 2400"
tex = '20\cdot 15\cdot 8 = 2400'

[area]
value = 1160.0
exact = "2(20·15 + 20·8 + 15·8) = 1160"
tex = '2(20\cdot 15 + 20\cdot 8 + 15\cdot 8) = 1160'
+++

An L-shaped block: the outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15) in the
xy plane, extruded 8 along +z. Its step is two faces: the riser x = 5, from y = 5 to y = 15, and the
tread y = 5, from x = 5 to x = 20, meeting in the inside edge along x = 5, y = 5.

Then edit the block directly: delete the riser and the tread together. Deleting a face takes it out
of the solid and closes the gap it leaves with the faces round it, each run on along its own surface
until they meet. The faces left beside them have never met: the far end y = 15, which the riser ran
into, and the far side x = 20, which the tread ran into. They run on, the end to x = 20 and the side
to y = 15, and meet in a new edge along x = 20, y = 15; the top and the bottom close in to fill the
corner.

The result is the box from the origin to (20, 15, 8).

**As a set:**

$$[0,20]\times[0,15]\times[0,8]$$
