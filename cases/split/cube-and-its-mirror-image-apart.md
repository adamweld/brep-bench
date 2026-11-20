+++
title = "A cube and its mirror image, apart"
tags = ["split", "mirror", "multi-body", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestMirrored.py::testMirroredOffsetCase"

[volume]
value = 2000.0
exact = "2·10³"
tex = '2\cdot 10^3'

[area]
value = 1200.0
exact = "2·6·10²"
tex = '2\cdot 6\cdot 10^2'
+++

A cube and its mirror image across a plane, with a gap between them, so the result is two separate solids.

The cube is [4, 14] × [0, 10] × [0, 10]. Mirror it across the plane x = 0, which gives the cube
[−14, −4] × [0, 10] × [0, 10]. The two lie a gap of 8 apart and do not touch; together they are the result,
two separate cubes (a single compound of two solids).

**As a set:**

$$\{\,4 \le |x| \le 14\,\} \times [0, 10] \times [0, 10]$$
