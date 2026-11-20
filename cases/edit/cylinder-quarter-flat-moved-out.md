+++
title = "A quarter of a cylinder, one flat moved out"
tags = ["edit", "move-face", "cylinder", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/H8, I1"

[volume]
value = 648.5012223706134
exact = "h(πR²/4 + (d√(R² − d²) + R²·asin(d/R))/2) = 150π + 9√91 + 300·asin(3/10), R = 10, h = 6, d = 3: the quarter disk plus the segment of the disk between y = −3 and y = 0, extruded"
tex = 'h\left(\tfrac{\pi R^2}{4} + \tfrac{d\sqrt{R^2-d^2} + R^2\arcsin(d/R)}{2}\right) = 150\pi + 9\sqrt{91} + 300\arcsin\tfrac{3}{10}'

[area]
value = 463.93276505717216
exact = "2·(25π + (3√91 + 100·asin(3/10))/2) + 6·(10(π/2 + asin(3/10)) + 13 + √91) = 80π + 9√91 + 78 + 160·asin(3/10): the two ends, the wall (arc of π/2 + asin(3/10)), the flat x = 0 of width 10 + 3 and the new flat y = −3 of width √91"
tex = '80\pi + 9\sqrt{91} + 78 + 160\arcsin\tfrac{3}{10}'
+++

A quarter of a cylinder: the part of the cylinder of radius 10 about the z axis, from z = 0 to z = 6, whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 90°, so the part with x ≥ 0 and y ≥ 0. Its faces are the wall, a quarter of a cylinder; a bottom and a top, quarter disks; and two flat rectangles, 10 by 6, in the half-planes y = 0, x ≥ 0 and x = 0, y ≥ 0, which meet along the z axis in a convex edge.

Then edit the solid directly: move the flat in the plane y = 0 out by 3 along its outward normal, −y, to the plane y = −3. The wall runs on round past the old edge to meet it, and the bottom and the top grow to follow; the other flat is lengthened from y = 0 to y = −3.

So the result is the cylinder cut to x ≥ 0 and y ≥ −3.

**As a set:**

$$\{\, x^2 + y^2 \le 10^2,\ x \ge 0,\ y \ge -3,\ 0 \le z \le 6 \,\}$$
