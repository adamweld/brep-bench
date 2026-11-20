+++
title = "A web rib with a straight edge running down over a rounded rim"
tags = ["feature", "rib", "revolve", "union", "cylinder", "torus", "fillet", "analytic"]
origin = "OCCT tests/feat/featrf/B4"

[volume]
value = 53945.43119560335
exact = "the shaft, π20²·30 − 2πM + π14²·20 + π8²·20 with M = r²(R − r/2) − (πr²/4)(R − r + 4r/(3π)), r = 4, R = 20, plus the rib, ∫₁₄¹⁸ (50 − h(ρ)) · 2ρ asin(2/ρ) dρ, where 2ρ asin(2/ρ) is the length of the arc of radius ρ inside the slab |y| ≤ 2 and h = 30 for ρ ≤ 16, h = 26 + √(16 − (ρ − 16)²) for ρ ≥ 16 (a definite integral, evaluated to 30 digits)"

[area]
value = 8988.573129072707
exact = "the shaft with its rim rounded, 400π + 2π·20·26 + 2π·4·((R − r)π/2 + r) + π(16² − 14²) + 2π·14·20 + π(14² − 8²) + 2π·8·20 + 64π, less the rib's foot, Q(16) − Q(14) on the shoulder and ∫ 4·2ρ asin(2/ρ) dθ over ρ = 16 + 4 cos θ from 16 to 18 on the rim, and its joint with the second cylinder's side, 20·28 asin(1/7), plus its top Q(18) − Q(14), its outer face (50 − h(18))·36 asin(1/9), and its two flat sides, 2∫₁₄¹⁸ (50 − h) ρ/√(ρ² − 4) dρ, with Q(R) = 2√(R² − 4) + R² asin(2/R) (definite integrals, evaluated to 30 digits)"
+++

A stepped shaft of three cylinders about the z axis: radius 20 from z = 0 to 30, radius 14 from
z = 30 to 50, and radius 8 from z = 50 to 70. The big cylinder's top outer edge, the circle ρ = 20, z = 30, is rounded to radius 4: a quarter of a
torus about the circle ρ = 16, z = 26, running from the big cylinder's side at z = 26 to its top
at ρ = 16. The flat ring from ρ = 14 to 16 at z = 30 is the
first shoulder, and the ring from ρ = 8 to 14 at z = 50 the second.

A web rib stands in the corner between the first shoulder and the second cylinder's side. Its
profile, in the half-plane y = 0, x > 0, is the vertical line ρ = 18, which reaches past the shoulder
down to the rounded rim and past the top of the second cylinder; turned about the z axis, it sweeps
the cylinder ρ = 18. The rib is the plate 4 thick, 2 to each side of the plane y = 0 on the +x side,
between the second cylinder's side and that cylinder, from the shaft up to z = 50: it stands on the
shoulder from ρ = 14 to 16, and from ρ = 16 to 18 it fills the space above the rounded rim, down to
the rim, so that its outer face is about 20.5 high at ρ = 18. Its top lies in the plane
z = 50 of the second shoulder, which it continues. The shaft and the rib are one solid.

**As a set:**

$$\bigl(\{\rho \le 20,\ 0 \le z \le 30\} \setminus \{\rho > 16,\ z > 26,\ (\rho - 16)^2 + (z - 26)^2 > 16\}\bigr) \cup \{\rho \le 14,\ 30 \le z \le 50\} \cup \{\rho \le 8,\ 50 \le z \le 70\} \cup \{\,|y| \le 2,\ x > 0,\ 14 \le \rho \le 18,\ 0 \le z \le 50\,\}$$
