+++
title = "A ball less a large cylinder whose axis lies off its centre"
tags = ["boolean", "cut", "sphere", "cylinder", "oblique"]
origin = "OCCT tests/bugs/modalg_7/bug25939"
settled = false
+++

A ball of radius 10 centred on the origin, less the cylinder of radius 9 whose axis is the line
x = 2, y = 0, parallel to the z axis, and which runs from z = −12 to z = 12, past the ball at both
ends.

Seen from above, the cylinder's circle (centre (2, 0), radius 9) lies inside the ball's equator
circle (radius 10) on the side where x is negative, 3 short of it at the nearest point (−7
against −10), and the two circles cross at (5.75, ±8.18…), the points where
x = 23/4 and x² + y² = 100. So what is left of the ball is a curved crescent: the part of the ball
whose plan lies between the two circles, thickest, 3 across, at x = −10 to −7 and thinning to
nothing at the two crossing points, with the ball's spherical surface outside and the cylinder's
concave wall inside, and flat ends nowhere: the crescent is closed top and bottom by the ball's
own surface.

Its volume has no closed form to hand, being the integral over the crescent of 2√(100 − ρ²) with
ρ the distance from the ball's centre, so the case is unsettled until two kernels agree.

**As a set:**

$$B_{10} \setminus \{\,(x - 2)^2 + y^2 \le 81\,\}$$
