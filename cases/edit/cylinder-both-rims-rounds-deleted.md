+++
title = "A cylinder's rounded rims deleted, top and bottom: the ends and the side meet again"
tags = ["edit", "delete-face", "fillet", "torus", "cylinder", "analytic"]

[volume]
value = 1130.9733552923256
exact = "π·6²·10 = 360π"
tex = '\pi\cdot 6^2\cdot 10 = 360\pi'

[area]
value = 603.1857894892403
exact = "2π·6² + 2π·6·10 = 192π"
tex = '2\pi\cdot 6^2 + 2\pi\cdot 6\cdot 10 = 192\pi'
+++

A cylinder of radius 6 on the z axis from z = 0 to z = 10, with both its rims rounded to radius 2.
The top round is a quarter of a torus whose tube has radius 2 about the circle of radius 4 at z = 8,
tangent to the top along the circle of radius 4 and to the side along the circle at z = 8; the
bottom round is its mirror image, about the circle of radius 4 at z = 2, tangent to the bottom along
the circle of radius 4 and to the side along the circle at z = 2.

Then edit it directly: delete both rounds together. Deleting a face takes it out of the solid and
closes the gap it leaves with the faces round it, each run on along its own surface until they
meet. The top and the bottom run on outward from radius 4, and the side up from z = 8 and down from
z = 2, until they meet in the circles of radius 6 at z = 10 and at z = 0.

The result is the cylinder of radius 6 from z = 0 to z = 10.

**As a set:**

$$\{\,x^2 + y^2 \le 6^2,\ 0 \le z \le 10\,\}$$
