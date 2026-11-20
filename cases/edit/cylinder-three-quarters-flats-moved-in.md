+++
title = "Three quarters of a cylinder, both flats moved in"
tags = ["edit", "move-face", "cylinder", "planar", "reentrant", "analytic"]
origin = "OCCT tests/offset/faces_type_i/I5, I6"

[volume]
value = 1151.326433987616
exact = "h·(75π − 100·asin(1/5) − 8√6 − 4) = 450π − 600·asin(1/5) − 48√6 − 24, R = 10, h = 6, d = 2: the disk less its part with x ≥ −2 and y ≥ −2, extruded"
tex = '450\pi - 600\arcsin\tfrac{1}{5} - 48\sqrt{6} - 24'

[area]
value = 783.9313739777062
exact = "2·(75π − 100·asin(1/5) − 8√6 − 4) + 6·(10(3π/2 − 2·asin(1/5)) + 2(4√6 + 2)) = 240π − 320·asin(1/5) + 32√6 + 16: the two ends, the wall (arc of 3π/2 − 2·asin(1/5)), and the two flats, each 4√6 + 2 wide"
tex = '240\pi - 320\arcsin\tfrac{1}{5} + 32\sqrt{6} + 16'
+++

Three quarters of a cylinder: the part of the cylinder of radius 10 about the z axis, from z = 0 to z = 6, whose angle about the z axis, φ, measured from +x toward +y, lies between 90° and 360°, so the disk less its quadrant x > 0, y > 0. Its faces are the wall, three quarters of a cylinder; a bottom and a top, three quarters of disks; and two flat rectangles, 10 by 6, in the half-planes x = 0, y ≥ 0 and y = 0, x ≥ 0, which meet along the z axis in a concave edge.

Then edit the solid directly: move each flat 2 inward along its normal, into the solid, the first to the plane x = −2 and the second to the plane y = −2. The wall is trimmed back to meet them, and the flats are extended until they meet each other, along the line x = y = −2.

So the result is the cylinder with everything at x ≥ −2 and y ≥ −2 taken away. Each flat runs from the line x = y = −2 to the wall, 4√6 + 2 wide.

**As a set:**

$$\{\, x^2 + y^2 \le 10^2,\ 0 \le z \le 6 \,\} \setminus \{\, x \ge -2,\ y \ge -2 \,\}$$
