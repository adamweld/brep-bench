+++
title = "A web rib with a rounded edge in a shaft's shoulder"
tags = ["feature", "rib", "revolve", "union", "cylinder", "torus", "fillet", "analytic"]
origin = "OCCT tests/feat/featrf/A6, OCCT tests/feat/featrf/A8"

[volume]
value = 50314.61538134653
exact = "the shaft, π20²·30 + π10²·40 − 2πM with M = r²(R − r/2) − (πr²/4)(R − r + 4r/(3π)), r = 2, R = 20 (Pappus, the rounded-off corner), plus the rib, ∫₀⁷ (Q(10 + √(49 − t²)) − Q(10)) dt, where Q(R) = 2√(R² − 4) + R² asin(2/R) is the area of {|y| ≤ 2, x > 0, ρ ≤ R} and t = z − 30 (a definite integral, evaluated to 30 digits)"

[area]
value = 8752.604938385059
exact = "the shaft with its rim rounded, π400 + 2π·20·28 + 2π·r((R − r)π/2 + r) + π(18² − 10²) + 2π·10·40 + 100π, less the rib's foot on the shoulder, Q(17) − Q(10), and its joint with the small cylinder's side, 7·20 asin(1/5), plus the rib's two flat sides, 2∫ over x from √96 to √285 of √(49 − (√(x² + 4) − 10)²) dx, and its toroidal face, ∫₀^{π/2} 7ρ·2 asin(2/ρ) dθ with ρ = 10 + 7 cos θ (definite integrals, evaluated to 30 digits)"
+++

A shaft of two cylinders about the z axis: radius 20 from z = 0 to 30, and on it radius 10 from
z = 30 to 70. The big cylinder's top outer edge, the circle ρ = 20, z = 30, is rounded to radius 2: a
quarter of a torus about the circle ρ = 18, z = 28, running from the big cylinder's side at z = 28 to
its top at ρ = 18. The flat ring from ρ = 10 to 18 at z = 30 is the shoulder.

A web rib stands in the corner between the shoulder and the small cylinder's side. Its profile, in
the half-plane y = 0, x > 0, is the quarter circle of radius 7 about the point (ρ, z) = (10, 30),
from (17, 30) on the shoulder to (10, 37) on the small cylinder's side; turned about the z axis,
the arc sweeps a torus. The rib is the plate 4 thick, 2 to each side of the plane y = 0 on the +x
side, between the shoulder, the small cylinder's side and that torus: the points there within 7 of
the circle ρ = 10, z = 30. Its two flat sides are the planes y = ±2. The rib stands clear of the
rounded edge, and the shaft and the rib are one solid.

**As a set:**

$$\bigl(\{\rho \le 20,\ 0 \le z \le 30\} \setminus \{\rho > 18,\ z > 28,\ (\rho - 18)^2 + (z - 28)^2 > 4\}\bigr) \cup \{\rho \le 10,\ 30 \le z \le 70\} \cup \{\,|y| \le 2,\ x > 0,\ \rho \ge 10,\ z \ge 30,\ (\rho - 10)^2 + (z - 30)^2 \le 49\,\}$$
