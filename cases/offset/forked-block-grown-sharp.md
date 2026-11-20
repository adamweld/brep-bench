+++
title = "A forked block grown by 1 with sharp edges"
tags = ["offset", "grow", "planar", "sharp-edge", "slot", "analytic"]
origin = "OCCT tests/offset/shape_type_i_c/I4, OCCT tests/offset/shape_type_i_c/I5"

[volume]
value = 877.3333333333334
exact = "(376/3)·7: the moved outline, 376/3 in area, extruded 5 + 2·1"
tex = '\tfrac{376}{3}\cdot 7'

[area]
value = 661.3333333333334
exact = "2·(376/3) + (176/3)·7: the top and bottom, and the moved outline's perimeter 176/3 times the height 7"
tex = '2\cdot\tfrac{376}{3} + \tfrac{176}{3}\cdot 7'
+++

A forked block: the polygon in the xy plane through (0, 0), (10, 0), (10, 4), (13, 8), (9, 8),
(6, 4), (4, 4), (1, 8), (−3, 8) and (0, 4), extruded from z = 0 to z = 5. It is a base 10 × 4 with
two arms that flare outward as they rise, their outer sides slanting from (10, 4) to (13, 8) and from
(0, 4) to (−3, 8), and between them a slot, 2 wide at its floor y = 4 and 8 wide at the top y = 8,
its walls slanting from (6, 4) to (9, 8) and from (4, 4) to (1, 8). The block has acute corners at
the arms' tops, (13, 8) and (−3, 8), and reentrant ones where the arms leave the base, at (10, 4)
and (0, 4), and at the slot's floor, (6, 4) and (4, 4).

Grow it by 1 with its edges kept sharp: move every face 1 outward along its normal, and extend the
moved faces until they meet, so that no round is added. Nothing closes: the slot's floor moves up to
y = 5 and its walls toward each other, leaving it 1 wide at the floor.

So the grown block is the extrusion, from z = −1 to z = 6, of the polygon through (−1, −1), (11, −1),
(11, 11/3), (15, 9), (8.5, 9), (5.5, 5), (4.5, 5), (1.5, 9), (−5, 9) and (−1, 11/3).

**As a set:**

$$\operatorname{poly}\bigl((-1,-1),(11,-1),(11,\tfrac{11}{3}),(15,9),(8.5,9),(5.5,5),(4.5,5),(1.5,9),(-5,9),(-1,\tfrac{11}{3})\bigr)\times[-1,6]$$
