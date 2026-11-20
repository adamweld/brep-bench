+++
title = "A rectangle extruded along a slanted direction"
tags = ["feature", "extrude", "oblique", "planar", "analytic"]
origin = "FreeCAD tests/src/Mod/Part/App/FeatureExtrusion.cpp::testExecuteDir"

[volume]
value = 1920
exact = "240·8: the rectangle's area times the height reached"
tex = '240\cdot 8'

[area]
value = 1040
exact = "2·240 + 2|(20,0,0)×(6,0,8)| + 2|(0,12,0)×(6,0,8)| = 480 + 320 + 240"
tex = '2\cdot 240 + 2\cdot 160 + 2\cdot 120'
+++

The rectangle from (−10, −6, 0) to (10, 6, 0), in the plane z = 0, extruded by the vector (6, 0, 8): every point of it moves along that vector, and the solid is what the rectangle sweeps. The direction is not square to the rectangle; the top is the same rectangle slid 6 along x, from (−4, −6, 8) to (16, 6, 8).

The result is a slanted prism of six flat faces: the two rectangles; two parallelograms in the planes y = ±6, 20 long and 10 along the slant; and two rectangles tilted over, in the planes through x = ±10 and the slant, 12 by 10.

**As a set:**

$$\{\,(x + 6s,\ y,\ 8s)\ :\ |x| \le 10,\ |y| \le 6,\ 0 \le s \le 1\,\}$$
