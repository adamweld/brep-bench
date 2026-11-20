+++
title = "A rounded square section swept along a parabola arc"
tags = ["sweep", "planar-path", "parabolic-path", "b-spline-section", "periodic", "analytic"]
origin = "OCCT pipe/standard/E7"

[volume]
value = 1377.3522896356
exact = "A·L = 30·20(√2 + ln(1 + √2)): the section is symmetric about its center (area 30), so it holds its area per unit length of the path whatever the bend (the factor 1 − κa averages out). The arc's length is 2∫₀²⁰ √(1 + x²/400) dx = 20(√2 + ln(1 + √2))"
tex = 'A L = 30\cdot 20\left(\sqrt2 + \ln(1 + \sqrt2)\right)'

[area]
value = 954.3012004406
exact = "P·L + 2A: the lateral surface holds the section's perimeter P = 12(1 + ln(1 + √2)/√2) per unit length of the path (the factor 1 − κa averages out), plus the two end faces: 240(1 + ln(1 + √2)/√2)(√2 + ln(1 + √2)) + 60"
tex = 'P L + 2A = 240\left(1 + \tfrac{\ln(1 + \sqrt2)}{\sqrt2}\right)\left(\sqrt2 + \ln(1 + \sqrt2)\right) + 60'
+++

A pipe whose section is a closed periodic B-spline, swept along an arc of a parabola.

The path is the arc of the parabola y = x²/40 in the xy plane from (−20, 10, 0) through its
vertex (0, 0, 0) to (20, 10, 0).

The section, at every point of the path, is the periodic (unclamped, closed) uniform quadratic
B-spline, non-rational, whose control polygon is the square of side 6 centered on the path point,
in the plane square to the path, with two sides parallel to the z axis: a square rounded all the
way to the middle of each side, its curve four parabolic arcs, each from the middle of one side
to the middle of the next, tangent to the sides there. Its corners are (3, 3), (−3, 3), (−3, −3)
and (3, −3) in coordinates (a, z), a measured along the path's unit normal in the xy plane. The
parabola is tightest at its vertex, radius of curvature 20, more than the section's half-width
of 3, so the pipe never folds. The two ends of the pipe are flat, each the section there.

**As a set:**

$$\{\,\gamma(s) + a\,n(s) + b\,e_z : (a, b) \in S,\ 0 \le s \le L\,\},\quad
\gamma(x) = (x, x^2/40, 0),\ -20 \le x \le 20$$

with $n$ the unit normal to $\gamma$ in the xy plane, $s$ the arc length, and $S$ the region the
section curve encloses, in coordinates $(a, b)$.
