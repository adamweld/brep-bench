+++
title = "A turned post's rounded shoulder deleted, its cone made turned a radian about the axis: the cylinder and the cone meet again"
tags = ["edit", "delete-face", "fillet", "torus", "cylinder", "cone", "analytic"]

[volume]
value = 634.6017160251382
exact = "πR²h + πH(R² + Rr + r²)/3 = 150π + 52π = 202π, R = 5, h = 6, r = 2, H = 4"
tex = '\pi R^2 h + \tfrac{1}{3}\pi H(R^2 + Rr + r^2) = 202\pi'

[area]
value = 389.55748904513433
exact = "πR² + 2πRh + π(R + r)√((R − r)² + H²) + πr² = 25π + 60π + 35π + 4π = 124π"
tex = '\pi R^2 + 2\pi R h + \pi(R + r)\sqrt{(R - r)^2 + H^2} + \pi r^2 = 124\pi'
+++

A turned post on the z axis: the cylinder of radius 5 from z = 0 to z = 6, and on it the cone frustum
from radius 5 at z = 6 to radius 2 at z = 10, made as one solid. Its shoulder, the circle of radius 5
at z = 6 where the cylinder and the cone meet, is rounded to radius 1: the round is a piece of a torus
tangent to the cylinder and to the cone.

Then edit it directly: delete the round. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The
cylinder runs on up and the cone down until they meet again in the circle of radius 5 at z = 6.

The result is the post as it was before its shoulder was rounded.

**As a set:**

$$\{\,\sqrt{x^2 + y^2} \le 5,\ 0 \le z \le 6\,\} \cup \{\,4\sqrt{x^2 + y^2} + 3z \le 38,\ 6 \le z \le 10\,\}$$

Here the cone frustum is made turned by 1.0 radians (57.2958°) about the z axis before the two are joined. That changes no point of the solid, but where each surface's own parameter lines start differs between the two surfaces, so the rim where they meet is two edges and its round two faces; both faces are deleted.
