+++
title = "A rounded square section carried round a sharp corner with a round joint"
tags = ["sweep", "planar-path", "corner", "round", "b-spline-section", "periodic", "analytic"]
origin = "OCCT pipe/specific/K2"

[volume]
value = 1791.5017672705
exact = "1800 − (1 − π/4)·198/5 = 1760.4 + 9.9π: each run holds the section's area, 30, per unit length of its 30 (1800 together); inside the corner the runs overlap in ∫w(z)² dz, w(z) the section's half-width at height z; the wedge outside the corner is the section's outer half turned a quarter about the z axis, (π/2)·½∫w² dz. ∫w² dz = 198/5: over the section's right half, the arcs (u, z) = (3(2t − t²), −3(1 − t²)) and its mirror, each give 99/5"
tex = '2\cdot 30\cdot 30 - \left(1 - \tfrac{\pi}{4}\right)\tfrac{198}{5} = 1760.4 + 9.9\pi'

[area]
value = 1220.4644362287
exact = "60P + 60 − (2 − π/2)M, P = 12(1 + ln(1 + √2)/√2) the section's perimeter and M = ∫u ds along the section's right half = 81/8 + (117√2/16)·ln(1 + √2): each run's side surface is P long per unit length (60P in all), less where it lies inside the other run (M each) and the inner halves of the end faces at the corner (inside the other run), plus the wedge's surface (π/2)M, plus the two far ends (30 each)"
tex = '60P + 60 - \left(2 - \tfrac{\pi}{2}\right)M,\quad M = \tfrac{81}{8} + \tfrac{117\sqrt2}{16}\ln(1 + \sqrt2)'
+++

A pipe whose section is a closed periodic B-spline, swept along a path with one sharp corner, the
section carried round the corner by turning it about the corner.

The path: two straight runs, from (−30, 0, 0) along +x to the origin, then turning a right angle
to the left and on along +y to (0, 30, 0).

The section, square to the path and centered on it, is the periodic (unclamped, closed) uniform
quadratic B-spline, non-rational, whose control polygon is the square of side 6 with two sides
parallel to the z axis: a square rounded all the way to the middle of each side, its curve four
parabolic arcs, each from the middle of one side to the middle of the next, tangent to the sides
there. At the start its control polygon has corners (−30, 3, 3), (−30, −3, 3), (−30, −3, −3) and
(−30, 3, −3). The section keeps its two sides parallel to z along both runs, as a guide curve
running above the path at a constant height would hold it.

Each run is the section swept straight along it, ending flat at the corner, in the plane through
the origin square to the run. Inside the corner the two runs overlap. The wedge they leave open
outside the corner (x ≥ 0, y ≤ 0) is filled by the section turned about the vertical line through
the corner, the z axis, a quarter turn from square to the first run to square to the second. The
two ends of the pipe, at (−30, 0, 0) and (0, 30, 0), are flat, each the section there.

**As a set:**

$$\{\,(x, y, z) : |z| \le 3,\ \bigl(x \in [-30, 0],\ |y| \le w(z)\bigr) \text{ or } \bigl(y \in [0, 30],\ |x| \le
w(z)\bigr) \text{ or } \bigl(x \ge 0,\ y \le 0,\ \sqrt{x^2 + y^2} \le w(z)\bigr)\,\}$$

with $w(z) = 3(2t - t^2)$, $t = \sqrt{1 - |z|/3}$, the section's half-width at height $z$.
