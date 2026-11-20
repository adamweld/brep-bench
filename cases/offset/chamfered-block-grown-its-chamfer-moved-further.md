+++
title = "A chamfered block grown by 0.5, its chamfer moved 2.5"
tags = ["offset", "grow", "move-face", "planar", "sharp-edge", "analytic"]
origin = "OCCT tests/offset/shape_type_i_c/Y3, OCCT tests/offset/shape_type_i_c/Y4"

[volume]
value = 725.375
exact = "(104 − 3/8)·7: the moved outline, the rectangle 13 × 8 less a corner triangle with legs 3/4 and 1, extruded 6 + 2·0.5"
tex = '\left(13\cdot 8 - \tfrac12\cdot\tfrac34\cdot 1\right)\cdot 7'

[area]
value = 497.75
exact = "2(104 − 3/8) + 41.5·7: the top and bottom, and the moved outline's perimeter, 13 + 7 + 1.25 + 12.25 + 8, times 7"
tex = '2\left(104 - \tfrac38\right) + 41.5\cdot 7'
+++

A block whose outline in the xy plane is the polygon through (0, 0), (12, 0), (12, 3), (9, 7) and
(0, 7), extruded from z = 0 to z = 6: a box 12 × 7 × 6 with one vertical edge chamfered, the chamfer
face running from (12, 3) to (9, 7) in the plane 4x + 3y = 57, its outward normal (4, 3)/5.

Grow it with its edges kept sharp: move every face 0.5 outward along its normal, except the chamfer,
which moves 2.5 outward along (4, 3)/5, to the plane 4x + 3y = 69.5; and extend the moved faces until
they meet, so that no round is added. The sides move to x = 12.5, x = −0.5, y = −0.5 and y = 7.5,
the base and top to z = −0.5 and z = 6.5. The chamfer, moved the further, cuts more of the corner:
it meets the side x = 12.5 at y = 6.5 and the side y = 7.5 at x = 11.75, so the side x = 12.5 is
shortened to run only up to y = 6.5.

So the grown block is the extrusion, from z = −0.5 to z = 6.5, of the polygon through (−0.5, −0.5),
(12.5, −0.5), (12.5, 6.5), (11.75, 7.5) and (−0.5, 7.5).

**As a set:**

$$\operatorname{poly}\bigl((-\tfrac12,-\tfrac12),(12.5,-\tfrac12),(12.5,6.5),(11.75,7.5),(-\tfrac12,7.5)\bigr)\times[-\tfrac12,6.5]$$
