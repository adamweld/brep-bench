+++
title = "A web rib bridging the waist between two cones"
tags = ["feature", "rib", "revolve", "union", "cone", "analytic"]
origin = "OCCT tests/feat/featrf/A7"

[volume]
value = 76434.68051739993
exact = "the two frusta, (πh/3)(r₁² + r₁r₂ + r₂²) for (24, 14, 30) and (14, 22, 40), plus the rib, ∫₁₅⁴⁵ (Q(ρ_c(z)) − Q(ρ_b(z))) dz with ρ_c = 19 − (z − 15)/15 and Q(R) = 2√(R² − 4) + R² asin(2/R) the area of {|y| ≤ 2, x > 0, ρ ≤ R} (a definite integral, evaluated to 30 digits)"

[area]
value = 11835.349773931057
exact = "the frusta's area, 576π + π(24 + 14)√(30² + 10²) + π(14 + 22)√(40² + 8²) + 484π, less the two bands of the body the rib covers, ∫ 2ρ_b asin(2/ρ_b) √(1 + ρ_b′²) dz over 15 to 45, plus the rib's outer band, ∫₁₅⁴⁵ 2ρ_c asin(2/ρ_c) √(1 + 1/225) dz, plus its two flat sides, 2∫₁₅⁴⁵ (√(ρ_c² − 4) − √(ρ_b² − 4)) dz (definite integrals, evaluated to 30 digits)"
+++

Two cone frusta stacked on the z axis, with a waist between them. The lower one narrows from
radius 24 at z = 0 to radius 14 at z = 30, and the upper one widens from radius 14 at z = 30 to
radius 22 at z = 70. So the body's radius is ρ_b(z) = 24 − z/3 for 0 ≤ z ≤ 30 and
ρ_b(z) = 14 + (z − 30)/5 for 30 ≤ z ≤ 70, and its ends are the discs of radius 24 at z = 0 and of
radius 22 at z = 70. The waist is the circle ρ = 14, z = 30, a concave crease.

A web rib bridges the waist. Its profile, in the half-plane y = 0, x > 0, is the straight line from
(ρ, z) = (19, 15) on the lower cone to (17, 45) on the upper one; turned about the z axis, the line
sweeps a third cone, ρ = 19 − (z − 15)/15. The rib is the plate 4 thick, 2 to each side of the plane
y = 0 on the +x side, between the body and that cone, from z = 15 to z = 45: where it leaves the
body at z = 15 and returns to it at z = 45 it has no thickness. Its two flat sides are the planes
y = ±2. The body and the rib are one solid.

**As a set:**

$$\{\,0 \le z \le 70,\ \rho \le \rho_b(z)\,\} \cup \{\,|y| \le 2,\ x > 0,\ 15 \le z \le 45,\ \rho_b(z) \le \rho \le 19 - \tfrac{z - 15}{15}\,\}$$
