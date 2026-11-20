+++
title = "A quarter of a cylinder, both flats moved in"
tags = ["edit", "move-face", "cylinder", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/H9, I2"

[volume]
value = 256.84863791067795
exact = "h·(25π − 100·asin(1/5) − 8√6 + 4) = 150π − 600·asin(1/5) − 48√6 + 24, R = 10, h = 6, d = 2: the part of the disk with x ≥ 2 and y ≥ 2, extruded"
tex = '150\pi - 600\arcsin\tfrac{1}{5} - 48\sqrt{6} + 24'

[area]
value = 249.27654940333932
exact = "2·(25π − 100·asin(1/5) − 8√6 + 4) + 6·(10(π/2 − 2·asin(1/5)) + 2(4√6 − 2)) = 80π − 320·asin(1/5) + 32√6 − 16: the two ends, the wall (arc of π/2 − 2·asin(1/5)), and the two flats, each 4√6 − 2 wide"
tex = '80\pi - 320\arcsin\tfrac{1}{5} + 32\sqrt{6} - 16'
+++

A quarter of a cylinder: the part of the cylinder of radius 10 about the z axis, from z = 0 to z = 6, whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 90°, so the part with x ≥ 0 and y ≥ 0. Its faces are the wall, a quarter of a cylinder; a bottom and a top, quarter disks; and two flat rectangles, 10 by 6, in the half-planes y = 0, x ≥ 0 and x = 0, y ≥ 0, which meet along the z axis in a convex edge.

Then edit the solid directly: move each flat 2 inward along its normal, the first to the plane y = 2 and the second to the plane x = 2. The wall is trimmed back to meet them, and the flats are trimmed against each other, meeting along the line x = y = 2.

So the result is the cylinder cut to x ≥ 2 and y ≥ 2. Each flat runs from the line x = y = 2 to the wall, 4√6 − 2 wide.

**As a set:**

$$\{\, x^2 + y^2 \le 10^2,\ x \ge 2,\ y \ge 2,\ 0 \le z \le 6 \,\}$$
