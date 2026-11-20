+++
title = "A round bead turned over a drum's rim"
tags = ["feature", "revolve", "union", "torus", "cylinder", "analytic"]
origin = "OCCT tests/feat/featrf/B3"

[volume]
value = 15677.78623262228
exact = "π·20²·10 + (5π/6)·M, where M = 18·18π + ∫_{−2}^{0} ((18 + √(36 − t²))² − 400)/2 dt is the first moment about the axis of the part of the disc outside the drum's cross-section (the half disc above z = 10, centroid 18 out, and the piece under it beyond ρ = 20)"

[area]
value = 4683.477441615752
exact = "the drum's bottom 400π, its wall 2π·20·10 less the 20·2 band the bead covers, (5π/6)·40, its top 400π less the bead's foot (5π/12)(400 − 144), plus the bead's round face (5π/6)·6·(18(π + asin(1/3)) + 2) (the integral of ρ = 18 + 6 cos θ over θ from −asin(1/3) to π), its flat underside (5π/12)((18 + √32)² − 400), and its two ends, 2A with A = 18π + ∫_{−2}^{0} (√(36 − t²) − 2) dt"
+++

A drum, the cylinder of radius 20 about the z axis from z = 0 to 10.

A bead is turned on its rim: the part of the disc of radius 6 about the point (x, z) = (18, 10) in
the plane y = 0 that lies above the line z = 8 (a little more than half the disc), turned about the
z axis from +x towards +y through 150°. The disc reaches out to ρ = 24, so the bead overhangs the
drum's wall: it lies on the drum's top from ρ = 12 to 20, and against its wall from z = 8 to 10,
and outside the wall it has a flat underside at z = 8 from ρ = 20 to 18 + √32 ≈ 23.66. Its round
surface is the torus about the circle ρ = 18, z = 10, and its two ends are flat, in the half-planes
at 0° and 150°. The drum and the bead are one solid.

**As a set:**

$$\{\rho \le 20,\ 0 \le z \le 10\} \cup \{\,0^\circ \le \varphi \le 150^\circ,\ (\rho - 18)^2 + (z - 10)^2 \le 36,\ z \ge 8\,\}$$
