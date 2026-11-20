+++
title = "A box shelled open at top and bottom, its wall centred on its faces: a rectangular tube"
tags = ["shell", "centred", "planar", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestThickness.py::testTwoOpeningsCreateCenteredThroughWall"

[volume]
value = 432
exact = "6·(12·10 − 8·6): the section, the outer rectangle 12 × 10 less the inner 8 × 6, times the height"
tex = '6\,(12\cdot 10 - 8\cdot 6)'

[area]
value = 576
exact = "outside 2(12 + 10)·6 = 264, inside 2(8 + 6)·6 = 168, two rims 2·72 = 144"
tex = '264 + 168 + 144'
+++

The box from the origin to (10, 8, 6), shelled to a wall 2 thick, open at both its top and bottom faces, z = 6 and z = 0, with the wall centred on the four side faces left: half of it, 1 thick, inside them and half outside.

The moved faces are extended until they meet, every edge kept sharp, and run on to the planes of the two openings, z = 0 and z = 6, where the wall ends in flat rims. Each side moves 1 out and 1 in.

So the solid is a rectangular tube 6 high: the box from (−1, −1, 0) to (11, 9, 6), less the box from (1, 1, 0) to (9, 7, 6), which is open through.

**As a set:**

$$\bigl([-1,11]\times[-1,9] \setminus (1,9)\times(1,7)\bigr) \times [0,6]$$
