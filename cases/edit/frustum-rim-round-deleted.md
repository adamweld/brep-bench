+++
title = "A cone frustum's rounded top rim deleted: the top and the cone meet again"
tags = ["edit", "delete-face", "fillet", "torus", "cone", "analytic"]

[volume]
value = 395.84067435231395
exact = "πh(R² + Rr + r²)/3 = 126π, R = 6, r = 3, h = 6"
tex = '\tfrac{1}{3}\pi h(R^2 + Rr + r^2) = 126\pi'

[area]
value = 331.0416671496403
exact = "π(R² + r²) + π(R + r)√((R − r)² + h²) = 45π + 27√5π"
tex = '\pi(R^2 + r^2) + \pi(R + r)\sqrt{(R - r)^2 + h^2} = 45\pi + 27\sqrt{5}\pi'
+++

A cone frustum on the z axis, its base the disk of radius 6 at z = 0 and its top the disk of radius 3
at z = 6, with its top rim rounded to radius 1: the round is a piece of a torus whose tube has radius
1 about the circle of radius (7 − √5)/2 at z = 5, tangent to the top and to the cone.

Then edit it directly: delete the round. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The top
runs on outward and the cone up until they meet in the circle of radius 3 at z = 6.

The result is the frustum from radius 6 at z = 0 to radius 3 at z = 6.

**As a set:**

$$\{\,\sqrt{x^2 + y^2} \le 6 - z/2,\ 0 \le z \le 6\,\}$$
