+++
title = "A box shelled open at the top, its wall centred on its faces"
tags = ["shell", "centred", "planar", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestThickness.py::testPlanarOpenShellIsCenteredOnRetainedFaces"

[volume]
value = 600
exact = "12·10·7 − 8·6·5: the outer block 12 × 10 × 7 less the cavity 8 × 6 × 5"
tex = '12\cdot 10\cdot 7 - 8\cdot 6\cdot 5'

[area]
value = 688
exact = "outside 12·10 + 2(12 + 10)·7 = 428, rim 120 − 48 = 72, inside 48 + 2(8 + 6)·5 = 188"
tex = '428 + 72 + 188'
+++

The box from the origin to (10, 8, 6), shelled to a wall 2 thick, open at its top face, z = 6, with the wall centred on the faces it is grown from: half of it, 1 thick, inside them, and half outside.

The moved faces are extended until they meet, every edge kept sharp, and run on up to the opening plane, z = 6, where the wall ends in a flat rim. The four sides and the bottom each move 1 out and 1 in.

So the solid is the box from (−1, −1, −1) to (11, 9, 6), less the box from (1, 1, 1) to (9, 7, 6).

**As a set:**

$$\bigl([-1,11]\times[-1,9]\times[-1,6]\bigr) \setminus \bigl([1,9]\times[1,7]\times[1,6]\bigr)$$
