+++
title = "A block grown by 1, half of one side moved 3"
tags = ["offset", "grow", "move-face", "planar", "sharp-edge", "analytic"]
origin = "OCCT tests/offset/shape_type_i_c_multi/A1"

[volume]
value = 2320.0
exact = "290·8: the moved outline, the rectangle 22 × 12 and a step 13 × 2, extruded 6 + 2·1"
tex = '\left(22\cdot 12 + 13\cdot 2\right)\cdot 8'

[area]
value = 1156.0
exact = "2·290 + 72·8: the top and bottom, and the moved outline's perimeter, 22 + 14 + 13 + 2 + 9 + 12, times 8"
tex = '2\cdot 290 + 72\cdot 8'
+++

The box from the origin to (20, 10, 6), its side y = 10 in two coplanar faces that meet along the
edge x = 8: the part with x ≤ 8, and the part with x ≥ 8.

Grow it by 1 with its edges kept sharp: move every face 1 outward along its normal, except the part
of the side y = 10 with x ≥ 8, which moves 3 outward, to y = 13; and extend the moved faces until
they meet, so that no round is added. The sides x = 0 and x = 20 and the side y = 0 move to x = −1,
x = 21 and y = −1, and the base and top to z = −1 and z = 7. The side y = 10 with x ≤ 8 moves to y = 11,
so the two parts of the side are now 2 apart and in different planes; the face joining them is the
plane x = 8, from y = 11 to y = 13, running across the height of the block, the moved edge's
sweep.

So the grown block is the extrusion, from z = −1 to z = 7, of the polygon through (−1, −1), (21, −1),
(21, 13), (8, 13), (8, 11) and (−1, 11).

**As a set:**

$$\operatorname{poly}\bigl((-1,-1),(21,-1),(21,13),(8,13),(8,11),(-1,11)\bigr)\times[-1,7]$$
