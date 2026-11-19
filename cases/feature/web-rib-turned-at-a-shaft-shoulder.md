+++
title = "A web rib in a shaft's shoulder, its edge turned about the axis"
tags = ["feature", "rib", "revolve", "union", "cylinder", "cone", "hyperbola", "analytic"]
origin = "OCCT tests/feat/featrf/A1"

[volume]
value = 50406.136369327433
exact = "π·20²·30 + π·10²·40 + V_rib = 16000π + V_rib; V_rib = ∫₃₀⁴⁰ (Q(ρ_c) − Q(10)) dz = (10/7)∫₁₀¹⁷ (Q(R) − Q(10)) dR, ρ_c = 17 − 0.7(z − 30) the cone's radius, Q(R) = 2√(R² − 4) + R² asin(2/R) the area of {|y| ≤ 2, x > 0, ρ ≤ R}, ∫Q dR = (4/3)R√(R² − 4) + (R³/3) asin(2/R) − (8/3) ln(R + √(R² − 4)): V_rib = 140.65391189074142788"
tex = '16000\pi + \tfrac{10}{7}\int_{10}^{17}\bigl(Q(R) - Q(10)\bigr)\,dR,\quad Q(R) = 2\sqrt{R^2 - 4} + R^2\arcsin\tfrac{2}{R},\quad \int Q\,dR = \tfrac43 R\sqrt{R^2 - 4} + \tfrac{R^3}{3}\arcsin\tfrac{2}{R} - \tfrac83\ln\bigl(R + \sqrt{R^2 - 4}\bigr)'

[area]
value = 8848.0864216422277
exact = "2800π (the shaft) − (Q(17) − Q(10)) (the rib's foot on the shoulder, 28.111099946266792333) − 10·20 asin(1/5) (its joint on the small side, the band |y| ≤ 2 from z = 30 to 40) + 2·(10/7)∫₁₀¹⁷ (√(R² − 4) − √96) dR (its two flat sides in y = ±2, each 35.494858925173070954; ∫√(R² − 4) dR = (R/2)√(R² − 4) − 2 ln(R + √(R² − 4))) + (√149/7)[R² asin(2/R) + 2√(R² − 4)]₁₀¹⁷ (its conical edge, ∫ρ·2 asin(2/ρ) along the cone's slant √149/7 per unit of ρ, 49.01995784479347275)"
tex = '2800\pi - \bigl(Q(17) - Q(10)\bigr) - 200\arcsin\tfrac15 + \tfrac{20}{7}\int_{10}^{17}\bigl(\sqrt{R^2 - 4} - \sqrt{96}\bigr)dR + \tfrac{\sqrt{149}}{7}\Bigl[R^2\arcsin\tfrac{2}{R} + 2\sqrt{R^2 - 4}\Bigr]_{10}^{17}'
+++

A stepped shaft with one web rib in the concave corner of its shoulder: a rib whose outer edge is a
straight line turned about the shaft's axis.

- **The shaft:** a cylinder of radius 20 on the z axis from z = 0 to z = 30, and on it a
  cylinder of radius 10 from z = 30 to z = 70. The big cylinder's top, the ring from ρ = 10 to
  20 at z = 30, is the shoulder.
- **The rib:** its profile is the segment in the half-plane y = 0, x > 0 from (x, z) = (17, 30) on
  the shoulder to (10, 40) on the small cylinder's side. Turned about the z axis, the segment
  sweeps a cone, ρ = 17 − 0.7(z − 30). The rib is the material between the shoulder, the small
  cylinder's side and that cone, within 2 of the plane y = 0 on the +x side: a web 4 thick whose two
  flat sides are the planes y = ±2 and whose outer edge is the cone, not a plane, so that the
  rib's flat sides are bounded there by arcs of hyperbolas.

The rib meets the shoulder from ρ = 10 to ρ = 17 and the small cylinder's side from z = 30 to
z = 40. The shaft and the rib are one solid.

**As a set:**

$$\{\rho \le 20,\ 0 \le z \le 30\} \cup \{\rho \le 10,\ 30 \le z \le 70\} \cup \{\, |y| \le 2,\ x > 0,\ \rho \ge 10,\ z \ge 30,\ 10(\rho - 10) + 7(z - 30) \le 70 \,\}$$
