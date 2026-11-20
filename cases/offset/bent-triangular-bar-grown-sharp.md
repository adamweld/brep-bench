+++
title = "A bent triangular bar grown by 1, its edges kept sharp"
tags = ["offset", "grow", "cone", "cylinder", "sharp-edge", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug427_4"

[volume]
value = 149.17045427424436
exact = "∫ from ρ₀ to 7 of ρ(ρ − ρ₀)(π/2 + 2 asin(1/ρ)) dρ, ρ₀ = 3 − √2: the grown triangle's section at radius ρ is ρ − ρ₀ tall, and the points at that radius with x ≥ −1 and y ≥ −1 span the angle π/2 + 2 asin(1/ρ)"
tex = '\int_{\rho_0}^{7} \rho(\rho - \rho_0)\left(\tfrac\pi2 + 2\arcsin\tfrac1\rho\right)d\rho,\quad \rho_0 = 3 - \sqrt2'

[area]
value = 215.18377405079675
exact = "with a(ρ) = π/2 + 2 asin(1/ρ): the bottom ∫ρ a dρ, the cone √2∫ρ a dρ (both from ρ₀ to 7), the cylinder 7(4 + √2)a(7), and the two flat ends 2∫(ρ − ρ₀)ρ/√(ρ² − 1) dρ (from ρ₀ to 7)"
tex = '(1 + \sqrt2)\int_{\rho_0}^{7}\rho\,a\,d\rho + 7(4 + \sqrt2)\,a(7) + 2\int_{\rho_0}^{7}\frac{(\rho - \rho_0)\rho}{\sqrt{\rho^2 - 1}}\,d\rho,\quad a = \tfrac\pi2 + 2\arcsin\tfrac1\rho'
+++

A short bar of triangular section bent through a quarter turn: the right triangle with corners
(ρ, z) = (4, 0), (6, 0) and (6, 2), in the half plane at φ = 0° (the xz plane, x > 0), turned 90°
about the z axis, right-handed, from +x toward +y, to the half plane at φ = 90°. Its faces are the
flat bottom, z = 0, a piece of annulus; the outside, the cylinder ρ = 6, 0 ≤ z ≤ 2; the slanting
cone ρ − z = 4 from (4, 0) to (6, 2); and two flat triangular ends, in the planes y = 0 and x = 0.

Grow it by 1 with its edges kept sharp: move each face 1 outward along its normal and extend the
moved faces until they meet, so that no round is added. The bottom moves to z = −1, the cylinder to
ρ = 7, the cone to ρ − z = 4 − √2, the end in y = 0 to y = −1 and the end in x = 0 to x = −1. The
grown section in (ρ, z) is the triangle with corners (3 − √2, −1), (7, −1) and (7, 3 + √2), and the
solid is the part of its turn about the z axis lying in x ≥ −1, y ≥ −1; the corner line x = y = −1
is clear of it, since 3 − √2 > √2.

**As a set:**

$$\{\,x \ge -1,\ y \ge -1,\ z \ge -1,\ \rho \le 7,\ \rho - z \ge 4 - \sqrt2\,\}$$
