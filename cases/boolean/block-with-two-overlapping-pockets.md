+++
title = "A block with two overlapping pockets of one depth"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "pocket"]
origin = "OCCT boolean/bcut_simple/H4"

[volume]
value = 1568.0
exact = "20·12·8 − 88·4"
tex = '20\cdot 12\cdot 8 - 88\cdot 4'

[area]
value = 1168.0
exact = "992 − 88 + 88 + 44·4"
tex = '992 - 88 + 88 + 44\cdot 4'
+++

A block from (0, 0, 0) to (20, 12, 8) with two flat-bottomed pockets cut from its top, each 4 deep, so that both floors lie in the plane z = 4: the first is the box from (2, 2, 4) to (10, 10, 9), and the second, partly overlapping it, the box from (6, 4, 4) to (16, 8, 9). Cut the first, then the second, from the block.

The pockets share the square 6 ≤ x ≤ 10, 4 ≤ y ≤ 8, and the second cuts away from a floor the first has already made, in the same plane: the result has one floor, in the plane z = 4, over the footprint of the two boxes together, which has area 64 + 40 − 16 = 88 and a boundary of length 44.

**As a set:**

$$[0,20]\times[0,12]\times[0,8] \setminus \bigl([2,10]\times[2,10]\times[4,9] \cup [6,16]\times[4,8]\times[4,9]\bigr)$$
