+++
title = "A frustum of an oblique cone"
tags = ["loft", "ruled", "cone", "oblique"]
origin = "OCCT modalg_2/bug22361"
settled = false
+++

A circle of radius 6 about the origin in the plane z = 0, and a circle of radius 3 about the point
(8, 0, 10) in the plane z = 10: their centres are the ends of a straight spine that leans from the
vertical, and both circles lie square to the z axis, not to the spine.

Join the point at angle θ of the lower circle, (6 cos θ, 6 sin θ, 0), to the point at the same
angle θ of the upper, (8 + 3 cos θ, 3 sin θ, 10), by a straight line, for every θ. These lines
sweep the solid's side. The two circles are similar and alike turned, so the lines all pass through
one point, (16, 0, 20), and the side is a piece of an oblique circular cone with that apex. The two
discs the circles bound are the ends.

The volume is that of any frustum of height 10 between circles of radius 6 and 3, 210π. The side's
area is an elliptic integral.

**As a set:**

$$\operatorname{conv}\bigl(\{\,x^2 + y^2 \le 36,\ z = 0\,\} \cup \{\,(x - 8)^2 + y^2 \le 9,\ z = 10\,\}\bigr)$$
