+++
title = "A block with a top pocket and a cut from below that meets its floor face to face"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "pocket"]
origin = "OCCT boolean/bcut_simple/J8, OCCT boolean/bcut_simple/K7"

[volume]
value = 1504
exact = "20·12·8 − 64·4 − 40·4"
tex = '20\cdot 12\cdot 8 - 64\cdot 4 - 40\cdot 4'

[area]
value = 1200
exact = "992 − 64 − 40 + 48 + 24 + 32·4 + 28·4"
tex = '992 - 64 - 40 + 48 + 24 + 32\cdot 4 + 28\cdot 4'
+++

A block from (0, 0, 0) to (20, 12, 8). Cut from its top the pocket box from (2, 2, 4) to (10, 10, 9), 4 deep; then cut from below the box from (6, 4, −1) to (16, 8, 4), whose top face lies in the plane of the pocket's floor, z = 4. They share no volume; their faces meet at z = 4 over the square 6 ≤ x ≤ 10, 4 ≤ y ≤ 8, area 16.

Over that square the two cuts meet and the result is open through the block there, a 4 by 4 window; elsewhere the pocket keeps what is left of its floor (area 64 − 16 = 48) and the cut from below what is left of its ceiling (40 − 16 = 24), both in z = 4.

**As a set:**

$$[0,20]\times[0,12]\times[0,8] \setminus \bigl([2,10]\times[2,10]\times[4,9] \cup [6,16]\times[4,8]\times[-1,4]\bigr)$$
