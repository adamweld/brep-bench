+++
title = "Two balls united a million from the origin"
tags = ["boolean", "union", "sphere", "analytic", "far-from-origin", "scale"]
origin = "brep-conformance 1.0.0, booleans/scale/far-from-origin-spheres"

[volume]
value = 765.9502088752257
exact = "4π(r₁³ + r₂³)/3 − πh₁²(3r₁ − h₁)/3 − πh₂²(3r₂ − h₂)/3, r₁ = 5, r₂ = 4, d = 7, h₁ = 6/7, h₂ = 8/7"
tex = '\tfrac{4}{3}\pi\left(r_1^3 + r_2^3\right) - \tfrac{\pi}{3}h_1^2(3r_1 - h_1) - \tfrac{\pi}{3}h_2^2(3r_2 - h_2),\quad r_1 = 5,\ r_2 = 4,\ h_1 = \tfrac{6}{7},\ h_2 = \tfrac{8}{7}'

[area]
value = 459.5701253251354
exact = "4π(r₁² + r₂²) − 2π(r₁h₁ + r₂h₂), r₁ = 5, r₂ = 4, h₁ = 6/7, h₂ = 8/7"
tex = '4\pi\left(r_1^2 + r_2^2\right) - 2\pi\left(r_1 h_1 + r_2 h_2\right),\quad r_1 = 5,\ r_2 = 4,\ h_1 = \tfrac{6}{7},\ h_2 = \tfrac{8}{7}'
+++

Two balls a million from the origin, united: a ball of radius 5 centered on
(600000, −800000, 0), and a ball of radius 4 centered on (600002, −799997, 6), 7 from the first
along the direction (2, 3, 6)/7, which no axis lines up with.

The solid is a few units across, but every coordinate of it is about 10⁶, so a kernel working
in absolute coordinates keeps some twenty fewer bits of its shape than it would at the origin. The
two spheres cross in a circle of radius √(25 − (29/7)²) ≈ 2.80, on the plane 29/7 from the first
center toward the second: the first ball loses a cap 6/7 high, the second one 8/7 high.

**As a set:**

$$B_5(c) \cup B_4\bigl(c + (2, 3, 6)\bigr),\quad c = (600000, -800000, 0)$$
