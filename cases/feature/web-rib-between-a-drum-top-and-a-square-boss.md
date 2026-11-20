+++
title = "A web rib between a drum's top and a square boss, its edge a cone"
tags = ["feature", "rib", "revolve", "union", "cone", "planar", "analytic"]
origin = "OCCT tests/feat/featrf/B1"

[volume]
value = 20722.322155816884
exact = "the drum and boss, π·20²·10 + 16·16·30, plus the rib, ∫_{−2}^{2} ∫_8^{√(400 − y²)} (20 − √(x² + y²))/0.6 dx dy (a definite integral, evaluated to 30 digits)"

[area]
value = 5889.581904631212
exact = "the drum, 400π + 400π + (400π − 256 − A) for its bottom, side and top less the boss's footprint and the rib's foot A = 2∫₀² (√(400 − y²) − 8) dy, the boss, 4·16·30 − 4·20 + 256 for its sides less the rib's joint and its top, the rib's cone, √(1 + 1/0.36)·A, and its two flat sides, 2∫₈^{√396} (20 − √(x² + 4))/0.6 dx (definite integrals, evaluated to 30 digits)"
+++

A drum, the cylinder of radius 20 about the z axis from z = 0 to 10, with a square boss on its top:
the box |x| ≤ 8, |y| ≤ 8, from z = 10 to 40.

A web rib stands on the drum's top against the boss's side x = 8. Its profile, in the half-plane
y = 0, x > 0, is the straight line from (ρ, z) = (20, 10) on the drum's rim up to (8, 30) on the
boss's side; turned about the z axis, the line sweeps a cone, ρ = 20 − 0.6(z − 10). The rib is the
plate 4 thick, 2 to each side of the plane y = 0, between the drum's top, the boss's side and that
cone, so it is 0 high at the rim and 20 high at the boss. Its two flat sides are the planes y = ±2.
The drum, the boss and the rib are one solid.

**As a set:**

$$\{\rho \le 20,\ 0 \le z \le 10\} \cup \bigl([-8, 8]^2 \times [10, 40]\bigr) \cup \{\,x \ge 8,\ |y| \le 2,\ z \ge 10,\ \rho + 0.6\,(z - 10) \le 20\,\}$$
