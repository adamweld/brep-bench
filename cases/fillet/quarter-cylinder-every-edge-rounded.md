+++
title = "A quarter cylinder with every edge rounded"
tags = ["fillet", "corner", "cylinder", "sphere", "torus", "planar", "analytic"]
origin = "OCCT blend/simple/B1"

[volume]
value = 608.418821611776902
exact = "Steiner on the body P × [r, 8 − r] grown by r, P the region x ≥ r, y ≥ r, ρ ≤ 9, r = 1: V = ah + (2a + ph)r + (πp/2 + πh)r² + 4πr³/3, h = 6, a = area of P = ∫ᵣˢ(√(81 − x²) − r)dx, s = √80, p = 2(s − r) + 9(π/2 − 2 arcsin(r/9))"
tex = 'ah + (2a + ph)r + \left(\tfrac{\pi p}{2} + \pi h\right)r^2 + \tfrac{4}{3}\pi r^3,\quad r = 1,\ h = 6'

[area]
value = 399.735998120184867
exact = "2a + ph + πpr + 2πhr + 4πr², with a, p, h, r as for the volume"
tex = '2a + ph + \pi p r + 2\pi h r + 4\pi r^2'
+++

A quarter of a cylinder of radius 10: the points with x ≥ 0, y ≥ 0, x² + y² ≤ 100 and 0 ≤ z ≤ 8.
It has nine edges: the one up the axis, from (0, 0, 0) to (0, 0, 8); the four straight ones along
the flat faces x = 0 and y = 0 at the bottom and at the top; the two quarter circles of radius 10
around the bottom and the top; and the two upright ones on the curved wall, at (10, 0) and at
(0, 10). Round all nine to radius 1.

Three rounds meet at each of the eight corners, and each corner is closed by a patch of the sphere of
radius 1 that touches the three faces there, as the rolling ball leaves it. The solid is the union
of every ball of radius 1 that fits inside the quarter cylinder.

**As a set:**

$$(K \ominus B_1) \oplus B_1,\qquad K = \{\, x \ge 0,\ y \ge 0,\ \rho \le 10,\ 0 \le z \le 8 \,\}$$
