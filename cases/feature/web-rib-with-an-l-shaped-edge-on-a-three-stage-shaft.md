+++
title = "A web rib with an L-shaped edge turned about a three-stage shaft's cone, the shaft's ends rounded"
tags = ["feature", "rib", "revolve", "union", "cylinder", "cone", "fillet", "analytic"]
origin = "OCCT tests/feat/featrf/A4, OCCT tests/feat/featrf/A5"

[volume]
value = 58646.813154119436
exact = "the shaft: 12000π + 14000π/3 + 2000π less the two rounded rims 2π(M(20, 2) + M(10, 2)), M(R, r) = r²(R − r/2) − (πr²/4)(R − r + 4r/(3π)) (the corner of side r less its quarter disc, its moment about the axis), = 58486.075863100298; plus the rib ∫₁₀¹⁴ (2ρ − 14)·2ρ·asin(2/ρ) dρ = 160.73729101913827 (at ρ the rib is 2ρ − 14 high, from the cone z = 70 − 2ρ up to z = 56, and 2 asin(2/ρ) of a turn wide, ρ·2 asin(2/ρ) dρ)"
tex = '12000\pi + \tfrac{14000\pi}{3} + 2000\pi - 2\pi\bigl(M(20,2) + M(10,2)\bigr) + \int_{10}^{14}(2\rho - 14)\,2\rho\arcsin\tfrac{2}{\rho}\,d\rho'

[area]
value = 8632.6967040849180
exact = "the shaft, 400π + 1200π + 30π√500 + 100π + 400π = 2100π + 30π√500, less for each of the two rounded rims (π(R² − (R − r)²) + 2πR r) and plus its round π² r (R − r + 2r/π), R = 20 and 10, r = 2: 8539.424407532035; plus the rib: its flat top ∫₁₀¹⁴ 2ρ asin(2/ρ) dρ, its outer wall 14·2 asin(1/7)·14, its two flat sides 2∫(2√(x² + 4) − 14)dx for x from √96 to √192, less the cone it covers √5∫₁₀¹⁴ 2ρ asin(2/ρ) dρ and the small cylinder’s band it covers 10·2 asin(1/5)·6: 93.27229655288293"
tex = '2100\pi + 30\pi\sqrt{500} - \sum_{R = 20, 10}\bigl(\pi(R^2 - (R - 2)^2) + 4\pi R - 2\pi^2(R - 2 + \tfrac{4}{\pi})\bigr) + A_{\mathrm{rib}},\quad A_{\mathrm{rib}} = 93.27229655288293'
+++

A three-stage shaft with a web rib in the flank of its cone.

- **The shaft:** the solid turned about the z axis from the half-section with corners (ρ, z) = (0, 0),
  (20, 0), (20, 30), (10, 50), (10, 70), (0, 70): a cylinder of radius 20 from z = 0 to 30, a cone
  frustum narrowing from radius 20 at z = 30 to radius 10 at z = 50, and a cylinder of radius 10 up
  to z = 70. Its two end rims, the circle of radius 20 at z = 0 and the circle of radius 10 at
  z = 70, are each rounded to radius 2.
- **The rib:** its edge is an L turned about the axis. The profile is two segments in the half-plane
  y = 0, x > 0: from (x, z) = (10, 56) on the small cylinder's side to (14, 56), and from there down
  to (14, 42) on the cone. Turned about the z axis the first sweeps a flat ring, the plane z = 56 from
  ρ = 10 to 14, and the second a cylinder of radius 14. The rib is the material between the cone, the
  small cylinder's side, that ring and that cylinder, within 2 of the plane y = 0 on the +x side: a
  web 4 thick whose two flat sides are the planes y = ±2. As a set it is
  { |y| ≤ 2, x > 0, 10 ≤ ρ ≤ 14, 70 − 2ρ ≤ z ≤ 56 }.

The rib meets the cone from ρ = 10 to ρ = 14 and the small cylinder's side from z = 50 to z = 56. It
is clear of the rounded rims. The shaft and the rib are one solid.
