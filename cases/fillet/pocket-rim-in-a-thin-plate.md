+++
title = "A thin plate with a half-round pocket in its side, the pocket's rim rounded"
tags = ["fillet", "cylinder", "planar", "concave", "analytic"]
origin = "OCCT tests/blend/simple/Q1"

[volume]
value = 1140.2335742063297
exact = "1200 − (πR²/2)(3/2) − (A·L + M·π), R = 5, r = 1/2, A = r²(1 − π/4), M = r³(5/6 − π/4), L = πR: the generalized Pappus, the section's lengths growing by 1 + t/R as the rim bends away from the material"

[area]
value = 1405.139289522075
exact = "600 (bottom) + 600 − πR²/2 − rL − πr²/2 (top) + 25 − 2A + 160 (the four sides) + L(3/2 − r) (wall) + πR²/2 (floor) + (πr/2)(L + πr(1 − 2/π)) (the round)"
+++

A thin plate, the box from the origin to (30, 20, 2). In its side x = 0 is a half-round pocket: the
part of the cylinder of radius 5 about the line x = 0, y = 10 (parallel to z) that lies in x ≥ 0,
cut from the plate's top z = 2 down to a flat floor at z = 1/2, leaving a floor 1/2 thick. The
pocket opens to the top and to the side x = 0. Its wall is a half cylinder 3/2 high, which meets
the top in a semicircle of radius 5 and the side x = 0 along two upright lines, at y = 5 and
y = 15.

Round the one edge where the pocket's wall meets the plate's top, the semicircle, to radius 1/2.
The round is the surface a ball of radius 1/2 sweeps rolling along it, touching the top and the
wall; the wall curves away from the material, so the ball's center runs round a semicircle of
radius 5.5 at z = 3/2. At the semicircle's ends (0, 5, 2) and (0, 15, 2) the wall is square to the
side x = 0, so the round ends in it as a quarter circle of radius 1/2. Everything else stays
sharp.

**As a set:**

$$[0, 30] \times [0, 20] \times [0, 2] \setminus \Bigl(\{\,\rho \le 5,\ x \ge 0,\ z \ge \tfrac12\,\} \cup \{\,5 \le \rho \le \tfrac{11}{2},\ x \ge 0,\ z \ge \tfrac32,\ (\rho - \tfrac{11}{2})^2 + (z - \tfrac32)^2 \ge \tfrac14\,\}\Bigr)$$

where ρ = √(x² + (y − 10)²).
