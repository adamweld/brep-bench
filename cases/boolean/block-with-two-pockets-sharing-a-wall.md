+++
title = "A block with a pocket and a second pocket abutting it along a wall"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "pocket"]
origin = "OCCT boolean/bcut_simple/K8"

[volume]
value = 1632
exact = "20·12·8 − 72·4"
tex = '20\cdot 12\cdot 8 - 72\cdot 4'

[area]
value = 1152
exact = "992 − 72 + 72 + 40·4"
tex = '992 - 72 + 72 + 40\cdot 4'
+++

A block from (0, 0, 0) to (20, 12, 8), with two flat-bottomed pockets cut from its top, each 4 deep, floors both in the plane z = 4: the first is the box from (2, 2, 4) to (8, 10, 9), the second the box from (8, 4, 4) to (14, 8, 9). They touch along the square x = 8, 4 ≤ y ≤ 8, 4 ≤ z ≤ 8 and share no volume. Cut the first, then the second, from the block.

The two openings are one: the result has a single pocket whose footprint is the two rectangles together, area 48 + 24 = 72, with a floor in z = 4 and walls along the outline, of length 28 + 20 − 2·4 = 40; the shared wall between them is gone.

**As a set:**

$$[0,20]\times[0,12]\times[0,8] \setminus \bigl([2,8]\times[2,10]\times[4,9] \cup [8,14]\times[4,8]\times[4,9]\bigr)$$
