+++
title = "A funnel: a lofted cone frustum shelled inward, open at both ends"
tags = ["shell", "inward", "cone", "loft", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug5805_32"

[volume]
value = 6247.703890953255
exact = "(πh/3)((r₀² + r₀r₁ + r₁²) − (q₀² + q₀q₁ + q₁²)), h = 30, r₀ = 40, r₁ = 4, qᵢ = rᵢ − √(1 + k²), k = (r₀ − r₁)/h = 1.2: the frustum less the frustum inside the cone moved 1 inward"
tex = '\tfrac{\pi h}{3}\left(\left(r_0^2 + r_0r_1 + r_1^2\right) - \left(q_0^2 + q_0q_1 + q_1^2\right)\right),\quad h = 30,\ r_0 = 40,\ r_1 = 4,\ q_i = r_i - \sqrt{1 + k^2},\ k = \tfrac{r_0 - r_1}{h}'

[area]
value = 12911.921374636728
exact = "π(r₀ + r₁ + q₀ + q₁)·h√(1 + k²) + π(r₀² − q₀² + r₁² − q₁²): the outer and inner cones and the two rims"
tex = '\pi\left(r_0 + r_1 + q_0 + q_1\right)h\sqrt{1 + k^2} + \pi\left(r_0^2 - q_0^2 + r_1^2 - q_1^2\right)'
+++

A cone frustum about the z axis, lofted through two circles about the axis joined by straight
lines: the circle of radius 40 in the plane z = 0 and the circle of radius 4 in the plane z = 30.
Its side is the cone ρ = 40 − 1.2z, and its ends are flat disks.

Shell it inward to a wall 1 thick, open at both ends, a funnel. The outside of the wall is the
frustum's side; the inside is the cone moved 1 inward along its normal, the parallel cone
ρ = 40 − 1.2z − √2.44 (√(1 + 1.2²) ≈ 1.562), which runs on from z = 0 to z = 30 to meet the planes
of the openings. The rims are flat annuli, in z = 0 between radii 40 − √2.44 ≈ 38.438 and 40, and in
z = 30 between radii 4 − √2.44 ≈ 2.438 and 4.

**As a set:**

$$\{\, 40 - 1.2z - \sqrt{2.44} \le \rho \le 40 - 1.2z,\ 0 \le z \le 30 \,\}$$
