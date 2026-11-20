+++
title = "A pointed cone's base moved out"
tags = ["edit", "move-face", "cone", "apex", "analytic"]
origin = "OCCT tests/offset/faces_type_i/G1"

[volume]
value = 804.247719318987
exact = "πρ²h/3 = π·8²·12/3 = 256π: the cone of base radius 8 and height 12"
tex = '\tfrac{\pi\rho^2 h}{3} = \tfrac{\pi\cdot 8^2\cdot 12}{3} = 256\pi'

[area]
value = 563.531478602194
exact = "πρL + πρ² = 8π√208 + 64π, ρ = 8, L = √(8² + 12²) = √208: the side and the base"
tex = '\pi\rho L + \pi\rho^2 = 8\pi\sqrt{208} + 64\pi,\quad \rho = 8,\ L = \sqrt{208}'
+++

The cone whose base, of radius 6, lies in the plane z = 0 centered on the origin and whose apex is
(0, 0, 9). Its faces are the cone and the flat base disk.

Then edit the solid directly: move the base 3 outward along its normal, down, to the plane z = −3.
The cone runs on below z = 0 to meet it, to a base of radius 8 about the same axis; the apex does
not move.

So the result is the cone with apex (0, 0, 9) and base of radius 8 in the plane z = −3, 12 high.

**As a set:**

$$\{\, \rho \le \tfrac{2}{3}(9 - z),\ -3 \le z \le 9 \,\}$$
