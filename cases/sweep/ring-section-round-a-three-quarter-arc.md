+++
title = "A rounded square section swept along a three-quarter circle"
tags = ["sweep", "planar-path", "circular-path", "b-spline-section", "periodic", "analytic"]
origin = "OCCT pipe/standard/E3"

[volume]
value = 1696.4600329385
exact = "540π: the section is symmetric about its center (area 30: a square of half-side 3 less a third of each corner's triangle, 36 − 6), so its centroid is on the path and Pappus gives A·R·θ = 30·12·(3π/2)"
tex = 'A R\theta = 30\cdot 12\cdot\tfrac{3\pi}{2} = 540\pi'

[area]
value = 1161.4946977420
exact = "Pappus: the section's perimeter P (four parabolic arcs, each 3(1 + ln(1 + √2)/√2) long, so P = 12(1 + ln(1 + √2)/√2)) times R·θ = 18π, plus the two end faces (area 30 each): 216π(1 + ln(1 + √2)/√2) + 60"
tex = 'P R\theta + 2A = 216\pi\left(1 + \tfrac{\ln(1 + \sqrt2)}{\sqrt2}\right) + 60'
+++

A pipe whose section is a closed periodic B-spline, swept along a circular arc.

The path is the arc of the circle of radius 12 about the z axis in the xy plane, from (12, 0, 0)
through (0, 12, 0) and (−12, 0, 0) to (0, −12, 0): three quarters of a turn, leaving the start
along +y.

The section, at every point of the path, is the periodic (unclamped, closed) uniform quadratic
B-spline, non-rational, whose control polygon is the square of side 6 centered on the path point,
in the plane square to the path, with two sides parallel to the z axis: a square rounded all the
way to the middle of each side, so its curve is four parabolic arcs, each from the middle of one
side to the middle of the next, tangent to the sides there. Its corners are (3, 3), (−3, 3),
(−3, −3) and (3, −3) in coordinates (r, z), r measured outward from the z axis. At the start,
the section's control polygon has corners (15, 0, 3), (9, 0, 3), (9, 0, −3) and (15, 0, −3). The
two ends of the pipe are flat, each the section there.

**As a set:**

$$\{\,\gamma(s) + a\,n(s) + b\,e_z : (a, b) \in S,\ 0 \le s \le \tfrac{3\pi}{2}\,\},\quad
\gamma(s) = 12(\cos s, \sin s, 0),\ n(s) = (\cos s, \sin s, 0)$$

with $S$ the region the section curve encloses, in coordinates $(a, b)$.
