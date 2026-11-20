+++
title = "A prism with slanted sides shelled open at the top, its wall centred on its faces"
tags = ["shell", "centred", "planar", "slanted", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestThickness.py::testCase5829RectoVersoThicknessOnRotatedFillet"

[volume]
value = 735.272429209974
exact = "12·(66.5 + 7√5) − 8·(42.5 − 5√5): the outer and inner sections, 66.5 + 7√5 and 42.5 − 5√5, times their lengths 12 and 8"
tex = '12(66.5 + 7\sqrt5) - 8(42.5 - 5\sqrt5) = 458 + 124\sqrt5'

[area]
value = 825.8823325799681
exact = "outer: bottom 12(13 + √5), slanted sides 2·12·(7√5/2), ends 2(66.5 + 7√5); rim 12(6 + √5) − 8(6 − √5); inner: floor 8(11 − √5), slanted sides 2·8·(5√5/2), ends 2(42.5 − 5√5)"
tex = '(289 + 110\sqrt5) + (24 + 20\sqrt5) + (173 + 22\sqrt5) = 486 + 152\sqrt5'
+++

A prism with slanted sides: the trapezoid with corners (0, 0), (12, 0), (9, 6) and (3, 6) in the plane y = 0, extruded along y from 0 to 10. Its sides slope in by 3 over a height of 6, so each meets the bottom at an angle other than a right angle (about 63.4° inside the trapezoid, 116.6° at the top).

Shell it to a wall 2 thick, open at its top face, z = 6, with the wall centred on the faces it is grown from: half of it, 1 thick, inside them, and half outside. Every face but the top moves 1 out and 1 in, along its normal: the bottom, the two slanted sides, and the two ends, y = 0 and y = 10. The moved faces are extended until they meet, every edge kept sharp, and run on up to the plane z = 6, where the wall ends in a flat rim.

So the solid is the outer prism less the inner prism. The outer one is bounded by z = −1, the planes 2x − z = −√5 and 2x + z = 24 + √5 (each slanted side moved 1 out), y = −1 and y = 11, cut off at z = 6; the inner one by z = 1, the planes 2x − z = √5 and 2x + z = 24 − √5 (each moved 1 in), y = 1 and y = 9, up to z = 6.

**As a set:**

$$\{\,2x - z \ge -\sqrt5,\ 2x + z \le 24 + \sqrt5,\ -1 \le z \le 6,\ -1 \le y \le 11\,\} \setminus \{\,2x - z > \sqrt5,\ 2x + z < 24 - \sqrt5,\ 1 < z \le 6,\ 1 < y < 9\,\}$$
