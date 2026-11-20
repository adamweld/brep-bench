+++
title = "An elliptic ring: an ellipse swept round a circle"
tags = ["sweep", "planar-path", "revolution", "analytic"]
origin = "OCCT pipe/standard/B2"

[volume]
value = 3553.0575843921691
exact = "2πR·πab, R = 12, a = 5, b = 3: the ellipse's area times the circle its center travels"
tex = '2\pi R\cdot\pi a b,\quad R = 12,\ a = 5,\ b = 3'

[area]
value = 1924.6903703387175
exact = "2πR·P, P = 4a·E(1 − b²/a²) = 20·E(16/25) the ellipse's perimeter (E the complete elliptic integral of the second kind): the part of the wall's area weighted by the ellipse's own offset from its center cancels by symmetry, so each element of the perimeter travels the circle of radius R; the integral ∫ 2π(12 + 3 cos θ)√(9 sin²θ + 25 cos²θ) dθ over a full turn gives the same"
tex = '2\pi R\,P,\quad P = 4a\,E\!\left(1 - \tfrac{b^2}{a^2}\right)'
+++

An ellipse swept once round a circle, with no ends: a ring of elliptic section.

The path is the circle of radius 12 about the z axis in the plane z = 0. The section, where the
path passes (12, 0, 0), is the ellipse in the plane y = 0 (square to the path there) centered on
(12, 0, 0), with semi-axis 3 along x (across the ring) and semi-axis 5 along z. The section
stays square to the circle and keeps its semi-axis of 3 along the radius from the z axis, so the
ring is the ellipse turned about the z axis.

**As a set:**

$$\Bigl\{\,\bigl(\tfrac{\rho - 12}{3}\bigr)^2 + \bigl(\tfrac{z}{5}\bigr)^2 \le 1\,\Bigr\}$$
