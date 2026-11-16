+++
title = "A cone frustum's two rounded rims deleted: its ends and its cone meet again"
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

A cone frustum on the z axis, from the disk of radius 6 at z = 0 to the disk of radius 3 at z = 6 (its side the cone
2r + z = 12), with both its rims rounded to radius 3/2: each round a piece of a torus of tube radius 3/2 tangent to the
end and to the cone.

Then edit it directly: delete both rounds together. Deleting a face takes it out of the solid and closes the gap it leaves with the
faces round it, each run on along its own surface until they meet. The top and the base run on outward and the cone up and down until they meet in the circles of radius 3 at z = 6 and radius 6 at z = 0.

The result is the frustum from radius 6 at z = 0 to radius 3 at z = 6.

**As a set:**

$$\{\,2\sqrt{x^2 + y^2} + z \le 12,\ 0 \le z \le 6\,\}$$
