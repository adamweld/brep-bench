+++
title = "A ball standing in a plate, its beveled root deleted: the plate and the ball meet again"
tags = ["edit", "delete-face", "chamfer", "cone", "sphere", "analytic"]

[volume]
value = 485.89966375522135
exact = "π·8²·2 + πh²(3R − h)/3 = 128π + 80π/3 = 464π/3, R = 3, h = 4"
tex = '\pi\cdot 8^2\cdot 2 + \tfrac{1}{3}\pi h^2(3R - h) = \tfrac{464}{3}\pi'

[area]
value = 552.9203070318035
exact = "π·8² + 2π·8·2 + (π·8² − π·8) + 2πRh = 176π"
tex = '\pi\cdot 8^2 + 2\pi\cdot 8\cdot 2 + (\pi\cdot 8^2 - 8\pi) + 2\pi R h = 176\pi'
+++

A plate, the disk of radius 8 about the z axis from z = −2 to z = 0, and on it the ball of radius 3 about
(0, 0, 1), joined to it: the ball stands up out of the plate's top through the circle of radius √8 at z = 0.
The concave edge where they meet is beveled: the frustum from radius 4 at z = 0 to radius 3 at z = 1 is joined
too, so a cone runs from the plate's top at radius 4 up and in to the ball's equator, the circle of radius 3
at z = 1, filling the groove.

Then edit it directly: delete the bevel. Deleting a face takes it out of the solid and closes the gap it
leaves with the faces round it, each run on along its own surface until they meet. The top runs on inward and
the ball down from its equator until they meet in the circle of radius √8 at z = 0.

The result is the plate and the ball joined, their edge sharp.

**As a set:**

$$\{\,x^2 + y^2 \le 64,\ -2 \le z \le 0\,\} \cup \{\,x^2 + y^2 + (z - 1)^2 \le 9\,\}$$
