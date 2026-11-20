+++
title = "A cylinder's end face drafted about a diameter, the pull across the axis"
tags = ["draft", "planar", "cylinder", "analytic"]
origin = "OCCT tests/draft/angle/A5"

[volume]
value = 6283.185307179586
exact = "πR²H, R = 10, H = 20 (the tilt adds as much on one side of the axis as it takes off the other)"
tex = '\pi R^2 H,\quad R = 10,\ H = 20'

[area]
value = 1888.0430151165871
exact = "πR² + 2πRH + πR²/cos 8°"
tex = '\pi R^2 + 2\pi R H + \frac{\pi R^2}{\cos 8^\circ}'
+++

A cylinder of radius 10 about the z axis, from z = 0 to z = 20. Draft its top face by 8°, with the
pull direction +x and the neutral plane x = 0: the top face lies along the pull direction already, so
it turns about the line where it crosses the neutral plane, the diameter x = 0, z = 20 of its circle,
until it leans 8° from the x direction. It rises toward +x, becoming the plane
z = 20 + x·tan 8°. The side and the floor are not drafted. The top is replaced by the tilted plane:
the side is extended up to it on the +x half, where the plane is above z = 20, and cut down to it
on the −x half, where it is below.

The result is the cylinder up to a slanted top: its side is 20 + x·tan 8° high at the point
(x, y) of the circle, and its top face is an ellipse through the diameter at z = 20.

**As a set:**

$$\{\,\rho \le 10,\ 0 \le z \le 20 + x\tan 8^\circ\,\}$$
