+++
title = "A U-shaped bar notched across its arms, the notch flush with the slot end"
tags = ["boolean", "cut", "analytic", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/H1"

[volume]
value = 960.0
exact = "(240 − 48)·6 − 2·6·4·4"
tex = '(240 - 48)\cdot 6 - 2\cdot 6\cdot 4\cdot 4'

[area]
value = 880.0
exact = "2·192 + 88·6 − 2·16"
tex = '2\cdot 192 + 88\cdot 6 - 2\cdot 16'
+++

A bar from z = 0 to z = 6 whose section is a U: the polygon with corners (0, 0), (20, 0), (20, 4), (8, 4), (8, 8), (20, 8), (20, 12) and (0, 12), a rectangle 20 by 12 with a slot 4 wide cut in from its end at x = 20 as far as x = 8. And a box from (8, −2, 2) to (14, 14, 10), which crosses both arms of the U right across their width and from the middle of their height to above them. Cut the box from the bar.

The box's side face x = 8 lies in the same plane as the closed end of the slot, so that the slot's end face and the box's face are coplanar over the slot's width, 4 ≤ y ≤ 8, 2 ≤ z ≤ 6, and the box's face is inside the bar's material, over the arms, for 0 ≤ y ≤ 4 and 8 ≤ y ≤ 12. The cut takes a notch 6 long, 4 wide and 4 deep out of the top of each arm, from x = 8 to x = 14, open on the arm's top and on its outer and inner sides, and leaves the arms 2 thick there.

**As a set:**

$$\bigl([0,20]\times[0,12]\times[0,6] \setminus [8,21]\times[4,8]\times[-1,7]\bigr) \setminus ([8,14]\times[-2,14]\times[2,10])$$
