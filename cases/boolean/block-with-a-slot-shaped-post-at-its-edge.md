+++
title = "A block with a slot-shaped post flush along one edge and touching the next"
tags = ["boolean", "union", "analytic", "planar", "cylinder", "tangent", "coincident"]
origin = "OCCT boolean/bfuse_simple/I6, OCCT boolean/bfuse_simple/I7, OCCT boolean/bfuse_simple/I8"

[volume]
value = 3133.6991118430775
exact = "30·20·5 + 3·(8·4 + π·2²)"
tex = '30\cdot 20\cdot 5 + 3\,(8\cdot 4 + \pi\cdot 2^2)'

[area]
value = 1785.6991118430775
exact = "2·(30·20 + 30·5 + 20·5) + 3·(2·8 + 2π·2)"
tex = '2\,(30\cdot 20 + 30\cdot 5 + 20\cdot 5) + 3\,(2\cdot 8 + 2\pi\cdot 2)'
+++

A block from (0, 0, 0) to (30, 20, 5), its faces planes. And a post 3 high standing on its top, from z = 5 to z = 8, whose footprint is a stadium (a slot shape): the points within 2 of the segment from (2, 2) to (10, 2), so a rectangle 8 by 4 with a half disc of radius 2 on each short end. Unite them.

The footprint spans x from 0 to 14 and y from 0 to 4 and lies in the block's top. Its straight side at y = 0 lies along the block's side at y = 0, so the post's side there is in the plane of that block face, and its round end at the left touches the plane x = 0 of the block's other side along one vertical line, at the point (0, 2), and goes no further. No volume overlaps. The union is the block with the post on it.

**As a set:**

$$[0,30]\times[0,20]\times[0,5] \cup \{\,d\bigl((x,y),\,[2,10]\times\{2\}\bigr) \le 2,\ 5 \le z \le 8\,\}$$
