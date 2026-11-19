+++
title = "A box with rounded corners shelled open at the top, its wall centred on its faces"
tags = ["shell", "centred", "cylinder", "planar", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestThickness.py testMixedPlanarAndCylindricalFacesRemainCentered"

[volume]
value = 7619.999879422587
exact = "21·A₆ − 19·A₄, A_r = 30·20 + 2r(30 + 20) + πr² the rectangle 30 × 20 grown by r: = 21(1200 + 36π) − 19(1000 + 16π) = 6200 + 452π"
tex = '21A_6 - 19A_4,\quad A_r = 600 + 100r + \pi r^2:\ = 6200 + 452\pi'

[area]
value = 7895.398103108741
exact = "2A₆ + 21P₆ + 19P₄, P_r = 2(30 + 20) + 2πr: the bottom, the outside, the rim A₆ − A₄, the inside and its floor A₄ = 6400 + 476π"
tex = '2A_6 + 21P_6 + 19P_4,\quad P_r = 100 + 2\pi r:\ = 6400 + 476\pi'
+++

A box −20 ≤ x ≤ 20, −15 ≤ y ≤ 15, 0 ≤ z ≤ 20 with its four upright edges rounded to radius 5,
shelled to a wall 2 thick, open at its top face, z = 20, with the wall centred on the faces it is
grown from: half of it, 1 thick, inside them, and half outside.

The moved faces are extended until they meet, every edge kept sharp, and run on up to the
opening's plane, z = 20, where the wall ends in a flat rim. Each flat side moves 1 out and 1 in;
each rounded corner becomes a quarter cylinder about the same axis, of radius 6 outside and 4
inside, still tangent to the sides beside it; and the bottom moves to z = −1 outside and z = 1
inside.

So the solid is the rounded box 42 × 32 with corners of radius 6, from z = −1 to z = 20, less the
rounded box 38 × 28 with corners of radius 4, from z = 1 up.

**As a set:**

$$\{\,d((x, y), C) \le 6,\ -1 \le z \le 20\,\} \setminus \{\,d((x, y), C) \le 4,\ z \ge 1\,\}, \quad C = [-15, 15] \times [-10, 10]$$
