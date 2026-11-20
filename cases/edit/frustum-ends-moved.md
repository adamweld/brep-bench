+++
title = "A cone frustum's ends moved, the top out and the bottom in"
tags = ["edit", "move-face", "cone", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/A5, OCCT tests/offset/faces_type_i/A6, OCCT tests/offset/faces_type_i/A7, OCCT tests/offset/faces_type_i/A8"

[volume]
value = 254.7308043285724
exact = "πh(r₁² + r₁r₂ + r₂²)/3, h = 7, r₁ = 5, r₂ = 3/2"
tex = '\tfrac{\pi h}{3}\left(r_1^2 + r_1 r_2 + r_2^2\right),\quad h = 7,\ r_1 = 5,\ r_2 = \tfrac32'

[area]
value = 245.42293494149839
exact = "π(r₁ + r₂)ℓ + πr₁² + πr₂², ℓ = √(h² + (r₁ − r₂)²) = √61.25: the side and the two ends"
tex = '\pi(r_1 + r_2)\ell + \pi r_1^2 + \pi r_2^2,\quad \ell = \sqrt{h^2 + (r_1 - r_2)^2} = \sqrt{61.25}'
+++

A cone frustum about the z axis, of radius 6 at its flat bottom in the plane z = 0 and radius 3 at
its flat top in the plane z = 6; its cone's apex would be at z = 12. Its faces are the side and two
flat disks.

Then edit the solid directly: move the top 3 outward along its normal, up to the plane z = 9, and
the bottom 2 inward along its normal, up to the plane z = 2. The cone runs on to meet them, so the
top shrinks to radius 3/2 and the bottom to radius 5.

So the result is the frustum of the same cone from z = 2, radius 5, to z = 9, radius 3/2.

**As a set:**

$$\{\, \rho \le 6\left(1 - \tfrac{z}{12}\right),\ 2 \le z \le 9 \,\}$$
