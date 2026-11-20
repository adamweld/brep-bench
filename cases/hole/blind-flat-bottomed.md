+++
title = "A block with a flat-bottomed blind hole, started above its top face"
tags = ["hole", "blind", "cut", "planar", "cylinder", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestHole.py::testStartOffset"

[volume]
value = 3830.3539967061512
exact = "20·20·10 − π·3²·6 = 4000 − 54π"
tex = '20\cdot 20\cdot 10 - \pi\cdot 3^2\cdot 6 = 4000 - 54\pi'

[area]
value = 1713.0973355292326
exact = "2·400 + 4·20·10 − π·3² + 2π·3·6 + π·3² = 1600 + 36π: the block, with the mouth taken out of the top, the hole's wall and its floor put in"
tex = '2\cdot 400 + 4\cdot 200 - \pi\cdot 3^2 + 2\pi\cdot 3\cdot 6 + \pi\cdot 3^2 = 1600 + 36\pi'
+++

A block, the box from the origin to (20, 20, 10), with a blind hole of radius 3 in its top face, its axis the vertical line through (10, 10). The hole is 6 deep, from the top face (z = 10) down to a flat floor at z = 4.

If you drill the hole from a start above the top face, make its length reach to the floor at z = 4: the depth is measured from the top face, not from the start.

**As a set:**

$$\bigl([0,20]^2\times[0,10]\bigr) \setminus \bigl(\{\,d \le 3\,\} \times [4, 10]\bigr),\quad d = \sqrt{(x-10)^2 + (y-10)^2}$$
