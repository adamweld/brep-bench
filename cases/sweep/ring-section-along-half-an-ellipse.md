+++
title = "A rounded square section swept along half an ellipse"
tags = ["sweep", "planar-path", "elliptic-path", "b-spline-section", "periodic", "analytic"]
origin = "OCCT pipe/standard/E4"

[volume]
value = 1531.6199318039
exact = "A·L = 30·40·E(0.64): the section is symmetric about its center (area 30), so its centroid is on the path, and a section square to a planar path, centroid on it, holds its area per unit length of the path whatever the bend (the bend's factor 1 − κa on a strip at offset a averages out). L = 2·20·E(m), m = 1 − (12/20)² = 0.64, E the complete elliptic integral of the second kind in the parameter m; L = 51.05399772680"
tex = 'A L = 30\cdot 40\,E(0.64),\quad E(m) = \int_0^{\pi/2}\sqrt{1 - m\sin^2\theta}\,d\theta'

[area]
value = 1054.4656526424
exact = "P·L + 2A: the lateral surface holds the factor 1 − κa per unit length at offset a, which averages out round the symmetric section, whose perimeter is P = 12(1 + ln(1 + √2)/√2); plus the two end faces (area 30 each): 480(1 + ln(1 + √2)/√2)·E(0.64) + 60"
tex = 'P L + 2A = 480\left(1 + \tfrac{\ln(1 + \sqrt2)}{\sqrt2}\right)E(0.64) + 60'
+++

A pipe whose section is a closed periodic B-spline, swept along half an ellipse.

The path is the half of the ellipse x²/20² + y²/12² = 1 in the xy plane that has y ≥ 0, from
(20, 0, 0) through (0, 12, 0) to (−20, 0, 0); at its start it leaves along +y.

The section, at every point of the path, is the periodic (unclamped, closed) uniform quadratic
B-spline, non-rational, whose control polygon is the square of side 6 centered on the path point,
in the plane square to the path, with two sides parallel to the z axis: a square rounded all the
way to the middle of each side, its curve four parabolic arcs, each from the middle of one side
to the middle of the next, tangent to the sides there. Its corners are (3, 3), (−3, 3), (−3, −3)
and (3, −3) in coordinates (a, z), a measured along the path's unit normal in the xy plane,
outward at the start. At the start the section's control polygon has corners (23, 0, 3),
(17, 0, 3), (17, 0, −3) and (23, 0, −3). The ellipse is tightest at its ends, radius of curvature
144/20 = 7.2, more than the section's half-width of 3, so the pipe never folds. The two ends of
the pipe are flat, each the section there.

**As a set:**

$$\{\,\gamma(s) + a\,n(s) + b\,e_z : (a, b) \in S,\ 0 \le s \le L\,\},\quad
\gamma(\phi) = (20\cos\phi, 12\sin\phi, 0),\ 0 \le \phi \le \pi$$

with $n$ the unit normal to $\gamma$ in the xy plane (outward), $s$ the arc length, and $S$ the
region the section curve encloses, in coordinates $(a, b)$.
