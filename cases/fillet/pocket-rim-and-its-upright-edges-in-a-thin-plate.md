+++
title = "A thin plate with a half-round pocket in its side, the pocket's rim and its two upright edges rounded together"
tags = ["fillet", "corner", "cylinder", "planar", "concave", "crease", "unsettled"]
origin = "OCCT tests/blend/simple/Q2"
settled = false
+++

A thin plate, the box from the origin to (30, 20, 2). In its side x = 0 is a half-round pocket: the
part of the cylinder of radius 5 about the line x = 0, y = 10 (parallel to z) that lies in x ≥ 0,
cut from the plate's top z = 2 down to a flat floor at z = 1/2, leaving a floor 1/2 thick. The
pocket opens to the top and to the side x = 0. Its wall is a half cylinder 3/2 high, which meets
the top in a semicircle of radius 5 and the side x = 0 along two upright lines, at y = 5 and
y = 15, square to the side.

Round three edges of the plate together:

- the semicircle where the wall meets the top, to radius 1/2: the surface a ball of radius 1/2 sweeps
  rolling along it, touching the top and the wall; the wall curves away from the material, so the
  ball's center runs round a semicircle of radius 5.5 at z = 3/2;
- the two upright edges where the wall meets the side x = 0, each from the floor z = 1/2 up to the top,
  to radius 1: a vertical surface, a circular arc in plan of radius 1 inside the material, tangent to
  the side x = 0 and to the pocket's cylinder (its center at (1, 10 − √35) for the edge at y = 5 and
  at (1, 10 + √35) for the edge at y = 15), running from the floor's level up to the top.

The top's own edges along the side x = 0 stay sharp, as does everything else. So at each top corner,
(0, 5, 2) and (0, 15, 2), the semicircle's round and the upright round do not close the corner with a
patch of their own: each runs the whole length of its edge, neither is shortened by the other, and
where they cross they meet in a crease, the curve where the two surfaces intersect. The plate is the
box less the pocket and what each round removes.

**As a set:** with ρ = √(x² + (y − 10)²),

$$[0, 30] \times [0, 20] \times [0, 2] \setminus \Bigl(\{\,\rho \le 5,\ x \ge 0,\ z \ge \tfrac12\,\} \cup \{\,5 \le \rho \le \tfrac{11}{2},\ x \ge 0,\ z \ge \tfrac32,\ (\rho - \tfrac{11}{2})^2 + (z - \tfrac32)^2 \ge \tfrac14\,\} \cup \{\,z \ge \tfrac12\,\} \times (U_- \cup U_+)\Bigr)$$

where U₋ = { 0 ≤ x ≤ 1, y_c ≤ y ≤ 10, ρ ≥ 5, (x − 1)² + (y − y_c)² ≥ 1 } with y_c = 10 − √35, and U₊ is its mirror image in the plane y = 10.

The case has no closed form worked out, so it is unsettled until two kernels agree.
