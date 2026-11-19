+++
title = "Two balls a few microns across, united"
tags = ["boolean", "union", "sphere", "analytic", "small", "scale"]
origin = "brep-conformance 1.0.0, booleans/scale/microns-to-metres-spheres"

[volume]
value = 7.659502088752258e-07
exact = "4π(r₁³ + r₂³)/3 − πh₁²(3r₁ − h₁)/3 − πh₂²(3r₂ − h₂)/3, r₁ = 0.005, r₂ = 0.004, h₁ = 0.006/7, h₂ = 0.008/7"
tex = '\tfrac{4}{3}\pi\left(r_1^3 + r_2^3\right) - \tfrac{\pi}{3}h_1^2(3r_1 - h_1) - \tfrac{\pi}{3}h_2^2(3r_2 - h_2),\quad r_1 = 0.005,\ r_2 = 0.004,\ h_1 = \tfrac{0.006}{7},\ h_2 = \tfrac{0.008}{7}'

[area]
value = 0.00045957012532513543
exact = "4π(r₁² + r₂²) − 2π(r₁h₁ + r₂h₂), r₁ = 0.005, r₂ = 0.004, h₁ = 0.006/7, h₂ = 0.008/7"
tex = '4\pi\left(r_1^2 + r_2^2\right) - 2\pi\left(r_1 h_1 + r_2 h_2\right),\quad r_1 = 0.005,\ r_2 = 0.004,\ h_1 = \tfrac{0.006}{7},\ h_2 = \tfrac{0.008}{7}'
+++

Two balls united, a thousand times smaller than `scale/balls-united-a-million-from-the-origin`
and at the origin: a ball of radius 0.005 centered on the origin, and a ball of radius 0.004
centered on (0.002, 0.003, 0.006), 0.007 from the first.

If a unit is a millimetre the solid is about 15 microns across, so a kernel with a fixed absolute
tolerance of 10⁻⁷ or more has only a few hundred tolerances across it, and the cap the second
ball cuts from the first is 0.006/7 ≈ 0.00086 high.

**As a set:**

$$B_{0.005} \cup B_{0.004}\bigl((0.002, 0.003, 0.006)\bigr)$$
