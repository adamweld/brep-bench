+++
title = "A block with two overlapping pockets of one depth, each breaking out through a side wall"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "pocket"]
origin = "OCCT boolean/bcut_simple/I3, OCCT boolean/bcut_simple/L1"

[volume]
value = 1472
exact = "20·12·8 − 112·4"
tex = '20\cdot 12\cdot 8 - 112\cdot 4'

[area]
value = 1120
exact = "992 − 112 + 112 − 32 − 16 + 44·4"
tex = '992 - 112 + 112 - 32 - 16 + 44\cdot 4'
+++

A block from (0, 0, 0) to (20, 12, 8), with two flat-bottomed pockets cut from its top, each 4 deep, so that both floors lie in the plane z = 4. The first is the box from (−2, 2, 4) to (8, 10, 9), which runs out through the x = 0 side wall; the second, partly overlapping it, is the box from (6, 4, 4) to (22, 8, 9), which runs out through the x = 20 side wall. Cut the first, then the second, from the block.

The pockets share the square 6 ≤ x ≤ 8, 4 ≤ y ≤ 8 and the second cuts from a floor the first has already made, in the same plane: the result has one floor, in the plane z = 4, over the two footprints together, which has area 64 + 56 − 8 = 112. Its walls are the footprint's outline except where it meets the sides x = 0 and x = 20 (length 44, 4 high), and each side wall is notched: the x = 0 wall by an 8 by 4 opening and the x = 20 wall by a 4 by 4 one.

**As a set:**

$$[0,20]\times[0,12]\times[0,8] \setminus \bigl([-2,8]\times[2,10]\times[4,9] \cup [6,22]\times[4,8]\times[4,9]\bigr)$$
