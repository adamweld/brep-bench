+++
title = "A notched plate grown by 1 with sharp edges, its slanted wall moved 16, past a jaw"
tags = ["offset", "grow", "move-face", "planar", "sharp-edge", "vanishing-face", "analytic"]
origin = "OCCT tests/offset/shape_type_i_c/XT1"

[volume]
value = 16046.163559439172
exact = "(42·32 − k²/4)·12, k = 41 − 16√5: the grown box less what is left of the notch, a right triangle with legs k and k/2"
tex = '\left(42\cdot 32 - \tfrac{k^2}{4}\right)\cdot 12,\quad k = 41 - 16\sqrt{5}'

[area]
value = 4489.09584154479
exact = "2(42·32 − k²/4) + 12(2(42 + 32) − k + k/2 + k√5/2): the top and bottom, and the sides, the triangle's leg k cut from the face x = 41 and its other leg and its hypotenuse added"
tex = '2\left(42\cdot 32 - \tfrac{k^2}{4}\right) + 12\left(2(42 + 32) - k + \tfrac{k}{2} + \tfrac{\sqrt{5}}{2}k\right)'
+++

A plate whose outline in the xy plane is the polygon through (0, 0), (40, 0), (40, 10), (25, 10),
(20, 20), (40, 20), (40, 30) and (0, 30), extruded from z = 0 to z = 10: a rectangle with a notch
cut into its side x = 40, between a lower jaw (y ≤ 10) and an upper jaw (y ≥ 20). The notch's inner
wall slants, from (25, 10) to (20, 20), in the plane 2x + y = 60.

Grow it with its edges kept sharp: move every face 1 outward along its normal, except the slanted
wall, which moves 16 outward, along (2, 1)/√5, into the notch; and extend the moved faces until
they meet, so that no round is added. The outside becomes the box from (−1, −1, −1) to (41, 31, 11),
the jaws' inner faces move to y = 11 and y = 19, and the wall to the plane 2x + y = 60 + 16√5
(≈ 95.78).

The wall moves past the lower jaw: at y = 11 it would lie at x ≈ 42.39, beyond the jaws' ends at
x = 41. So the lower jaw's inner face is gone, overtaken, and the wall runs on to meet the end
face x = 41 directly, at y = 16√5 − 22 ≈ 13.78. What is left of the notch is a groove whose section
is a right triangle, between the wall, the upper jaw's face y = 19 from x = (41 + 16√5)/2 ≈ 38.39
to 41, and the opening in the face x = 41 from y ≈ 13.78 to 19.

**As a set:**

$$\bigl([-1, 41] \times [-1, 31] \times [-1, 11]\bigr) \setminus \{\, 11 \le y \le 19,\ 2x + y \ge 60 + 16\sqrt{5} \,\}$$
