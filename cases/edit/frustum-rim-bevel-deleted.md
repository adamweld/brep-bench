+++
title = "A cone frustum's beveled top rim deleted: the top and the cone meet again"
tags = ["edit", "delete-face", "chamfer", "cone", "analytic"]

[volume]
value = 395.84067435231395
exact = "πh(R² + Rr + r²)/3 = 126π, R = 6, r = 3, h = 6"
tex = '\tfrac{1}{3}\pi h(R^2 + Rr + r^2) = 126\pi'

[area]
value = 331.0416671496403
exact = "π(R² + r²) + π(R + r)√((R − r)² + h²) = 45π + 27√5π"
tex = '\pi(R^2 + r^2) + \pi(R + r)\sqrt{(R - r)^2 + h^2} = 45\pi + 27\sqrt{5}\pi'
+++

A cone frustum on the z axis, its base the disk of radius 6 at z = 0, its side the cone 2r + z = 12, cut off at
z = 6 where its top would be the disk of radius 3; but its top rim is beveled: the side stops at z = 5 (radius
3.5), and a steeper cone runs from there to the top, the disk of radius 2 at z = 6. That is the frustum from
radius 6 at z = 0 to 3.5 at z = 5 joined to the frustum from radius 3.5 at z = 5 to 2 at z = 6.

Then edit it directly: delete the bevel. Deleting a face takes it out of the solid and closes the gap it
leaves with the faces round it, each run on along its own surface until they meet. The top runs on outward
from radius 2 and the side up from z = 5 until they meet in the circle of radius 3 at z = 6.

The result is the frustum from radius 6 at z = 0 to radius 3 at z = 6.

**As a set:**

$$\{\,2\sqrt{x^2 + y^2} + z \le 12,\ 0 \le z \le 6\,\}$$
