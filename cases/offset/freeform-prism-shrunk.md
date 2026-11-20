+++
title = "A prism on a closed Bézier curve, shrunk by 1/2"
tags = ["offset", "shrink", "bezier", "freeform", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug5805_17, OCCT tests/bugs/modalg_2/bug5805_21"

[volume]
value = 505.98557353583753
exact = "(A − p·t + π·t²)(h − 2t), A = 442/5, p = 4∫₀¹|B′(s)|ds ≈ 33.80348960226989, h = 8, t = 1/2: the section shrunk by t, kept to the height left"
tex = '\left(A - p\,t + \pi t^2\right)(h - 2t),\quad A = \tfrac{442}{5},\ p = 4\int_0^1 |B^{\prime}(s)|\,ds,\ h = 8,\ t = \tfrac12'

[area]
value = 359.20058536528566
exact = "2(A − p·t + π·t²) + (p − 2πt)(h − 2t): the shrunk top and bottom, and the shrunk side"
tex = '2\left(A - p\,t + \pi t^2\right) + (p - 2\pi t)(h - 2t)'
+++

A prism on a closed curve of four cubic Bézier arcs. The first arc has control points (5, 0),
(5, 4), (4, 5), (0, 5), from the point (5, 0) to the point (0, 5), leaving (5, 0) straight up and
arriving at (0, 5) straight in toward −x. The other three are that arc turned 90°, 180° and 270°
about the z axis, so that the four make one smooth closed curve, a square with swollen sides, and its
inside is convex. The prism is the inside of that curve in the plane z = 0 extruded straight up to
z = 8: a flat bottom, a flat top and one freeform side of four patches. Its section has area 442/5
and its curve length is p = 4∫|B′| ≈ 33.80349 (B′ the derivative of the first arc).

Shrink it by 1/2: the solid of every point that is at least 1/2 from the outside of the prism, the
whole solid moved inward with no face opened. The flat top and bottom move 1/2 in, the side moves
1/2 inward along its normal, onto the surface offset 1/2 from it, and the moved faces are extended or
trimmed where they meet, so that the edges stay sharp, as they are on the prism. The curve's radius
of curvature is nowhere less than 1/2, so the shrunk section is the inside parallel curve at 1/2, of
length p − 2π·(1/2), and the result is a prism from z = 1/2 to z = 7½ on it, of the same kind.

**As a set:**

$$S \ominus B_{1/2}, \quad S = \{\,(x, y) \in \text{the curve's inside}\,\} \times [0, 8]$$
