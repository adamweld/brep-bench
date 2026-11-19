+++
title = "A square bar's end chamfered by a cone"
tags = ["boolean", "common", "cone", "planar", "analytic"]
origin = "OCCT tests/boolean/volumemaker/E5"

[volume]
value = 4798.826908213251
exact = "4a²h − (4a²(s₂ − s₁) − π(s₂³ − s₁³)/3 + 4(G(s₂) − G(s₁)))/√3, G(s) = s³·acos(a/s)/3 − (2a/3)s√(s² − a²) + (a³/3)·ln(s + √(s² − a²)), a = 10, h = 12, s₁ = 13, s₂ = a√2: at each height the square less the disk of radius s = 13 + √3(12 − z) in it is cut away, the disk's four segments beyond the sides r²·acos(a/r) − a√(r² − a²) integrated in s"
tex = '4a^2h - \tfrac{1}{\sqrt3}\left(4a^2(s_2 - s_1) - \tfrac{\pi}{3}\left(s_2^3 - s_1^3\right) + 4\left(G(s_2) - G(s_1)\right)\right),\quad G(s) = \tfrac{s^3}{3}\arccos\tfrac{a}{s} - \tfrac{2a}{3}s\sqrt{s^2 - a^2} + \tfrac{a^3}{3}\ln\left(s + \sqrt{s^2 - a^2}\right),\quad a = 10,\ h = 12,\ s_1 = 13,\ s_2 = a\sqrt2'

[area]
value = 1756.4411910749782
exact = "4a² + E + (2/√3)(4a² − E) + 4(2ah − (2/√3)(P(a) − P(y₁) − s₁(a − y₁))), E = πs₁² − 4(s₁²·acos(a/s₁) − ay₁), y₁ = √(s₁² − a²) = √69, P(y) = (y√(a² + y²) + a²·asinh(y/a))/2: the bottom; the end face E, the square within the circle of radius 13; the cone, over the rest of the square, at 30° (so 2/√3 times its plan); and the four sides, each less the sliver above the hyperbola where the cone cuts it"
tex = '4a^2 + E + \tfrac{2}{\sqrt3}\left(4a^2 - E\right) + 4\left(2ah - \tfrac{2}{\sqrt3}\left(P(a) - P(y_1) - s_1(a - y_1)\right)\right),\quad E = \pi s_1^2 - 4\left(s_1^2\arccos\tfrac{a}{s_1} - a y_1\right),\ y_1 = \sqrt{s_1^2 - a^2},\ P(y) = \tfrac{1}{2}\left(y\sqrt{a^2 + y^2} + a^2\operatorname{arsinh}\tfrac{y}{a}\right)'
+++

A square bar, the box from (−10, −10, 0) to (10, 10, 12), and a cone about the z axis that passes
through the circle of radius 13 in the bar's end face, z = 12, and makes 30° with that face,
widening downward: its radius at height z is 13 + √3(12 − z). Their common part: the bar's end
turned with a 30° chamfer, as a square nut's corners are.

The cone cuts only the upper ends of the bar's four vertical edges. The end face is the square
less its four corners outside the circle of radius 13, and each side face loses a sliver above
the hyperbola in which the cone cuts it, down to z = 12 − (10√2 − 13)/√3 ≈ 11.340 at the bar's
edges.

**As a set:**

$$\bigl([-10, 10] \times [-10, 10] \times [0, 12]\bigr) \cap \{\, \rho \le 13 + \sqrt3\,(12 - z) \,\}$$
