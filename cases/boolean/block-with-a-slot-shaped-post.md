+++
title = "A block with a slot-shaped post standing on its top"
tags = ["boolean", "union", "analytic", "planar", "cylinder", "coincident"]
origin = "OCCT boolean/bfuse_simple/E1"

[volume]
value = 3706.1946710584651
exact = "30·20·5 + 8·(10·6 + π·3²)"
tex = '30\cdot 20\cdot 5 + 8\,(10\cdot 6 + \pi\cdot 3^2)'

[area]
value = 2010.7964473723101
exact = "2·(30·20 + 30·5 + 20·5) + 8·(2·10 + 2π·3)"
tex = '2\,(30\cdot 20 + 30\cdot 5 + 20\cdot 5) + 8\,(2\cdot 10 + 2\pi\cdot 3)'
+++

A block from (0, 0, 0) to (30, 20, 5), its faces planes. And a post 8 high standing on its top, from z = 5 to z = 13, whose footprint is a stadium (a slot shape): the points within 3 of the segment from (10, 10) to (20, 10), so a rectangle 10 by 6 with a half disc of radius 3 on each short end. Unite them.

The post's footprint lies wholly inside the block's top, clear of its edges (it spans x from 7 to 23 and y from 7 to 13), and no volume overlaps. The union is the block with the post on it; the post's bottom face is the patch of the top it covers and no longer shows.

**As a set:**

$$[0,30]\times[0,20]\times[0,5] \cup \{\,d\bigl((x,y),\,[10,20]\times\{10\}\bigr) \le 3,\ 5 \le z \le 13\,\}$$
