+++
title = "A web rib with a quarter-arc edge reaching over a shaft's rounded rim"
tags = ["feature", "rib", "revolve", "union", "cylinder", "torus", "fillet", "analytic"]
origin = "OCCT tests/feat/featrf/A6, OCCT tests/feat/featrf/A8"

[volume]
value = 50008.03829960431
exact = "the shaft, π20²·30 + π10²·40 − 2πM with M = r²(R − r/2) − (πr²/4)(R − r + 4r/(3π)), r = 4, R = 20 (Pappus, the rounded-off corner), plus the rib, ∫₁₀^{17} (u(ρ) − l(ρ)) · 2ρ asin(2/ρ) dρ, where 2ρ asin(2/ρ) is the length of the arc of radius ρ inside the slab |y| ≤ 2, u = 30 + s, l = 30 for ρ ≤ 16, l = max(h, 30 − s) beyond, s = √(49 − (ρ − 10)²) and h = 26 + √(16 − (ρ − 16)²) the rounded rim's height; the break is at ρ* = 16.99885289… where s = 30 − h (definite integrals, evaluated to 30 digits)"

[area]
value = 8639.950863747048
exact = "the shaft with its rim rounded, 400π + 2π·20·26 + 2π·4·((R − r)π/2 + r) + π(16² − 10²) + 2π·10·40 + 100π, less the rib's foot on the shoulder, Q(16) − Q(10), with Q(R) = 2√(R² − 4) + R² asin(2/R), and on the rim, ∫ 4·2ρ asin(2/ρ) dθ over ρ = 16 + 4 cos θ from 16 to ρ*, and its joint with the small cylinder's side, 7·20 asin(1/5), plus the rib's two flat sides, 2∫ (u − l) ρ/√(ρ² − 4) dρ from 10 to 17, and its toroidal face, ∫ 7·2ρ asin(2/ρ) dψ over ρ = 10 + 7 cos ψ from the meeting point (ψ = −acos((ρ* − 10)/7)) to π/2 (definite integrals, evaluated to 30 digits)"
+++

A shaft of two cylinders about the z axis: radius 20 from z = 0 to 30, and on it radius 10 from
z = 30 to 70. The big cylinder's top outer edge, the circle ρ = 20, z = 30, is rounded to radius 4: a quarter of a
torus about the circle ρ = 16, z = 26, running from the big cylinder's side at z = 26 to its top
at ρ = 16. The flat ring from ρ = 10 to 16 at z = 30 is the shoulder.

A web rib stands in the corner between the shoulder and the small cylinder's side. Its profile, in
the half-plane y = 0, x > 0, is the circle of radius 7 about the point (ρ, z) = (10, 30), taken from
(10, 37) on the small cylinder's side round through (17, 30) and on until it meets the shaft again,
just below z = 30 over the rounded rim; turned about the z axis, that circle sweeps a torus. The rib
is the plate 4 thick, 2 to each side of the plane y = 0 on the +x side, of all the points within 7 of
the circle ρ = 10, z = 30 that the shaft does not already hold. So it fills the corner under the arc,
reaches out over the round, where it ends in a thin tip at ρ = 17, and its two flat sides are the
planes y = ±2. The shaft and the rib are one solid.

**As a set:**

$$\bigl(\{\rho \le 20,\ 0 \le z \le 30\} \setminus \{\rho > 16,\ z > 26,\ (\rho - 16)^2 + (z - 26)^2 > 16\}\bigr) \cup \{\rho \le 10,\ 30 \le z \le 70\} \cup \{\,|y| \le 2,\ x > 0,\ (\rho - 10)^2 + (z - 30)^2 \le 49\,\}$$
