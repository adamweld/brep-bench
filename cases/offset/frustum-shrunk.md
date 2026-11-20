+++
title = "A cone frustum shrunk by 1"
tags = ["offset", "shrink", "cone", "analytic"]
origin = "OCCT tests/offset/shape_type_a/A4, tests/offset/shape_type_i/A4"

[volume]
value = 10.274198422681729
exact = "πh(r₁² + r₁r₂ + r₂²)/3 with h = 1, r₁ = (10 − √13)/3, r₂ = (8 − √13)/3: the frustum of the same cone from z = 1 to z = 2"
tex = '\tfrac{\pi}{3}\,(r_1^2 + r_1 r_2 + r_2^2),\quad r_1 = \tfrac{10 - \sqrt{13}}{3},\ r_2 = \tfrac{8 - \sqrt{13}}{3}'

[area]
value = 34.59245266713605
exact = "π(r₁² + r₂²) + π(r₁ + r₂)·√13/3: the two ends and the side, of slant length √(1 + 4/9) = √13/3"
tex = '\pi(r_1^2 + r_2^2) + \pi(r_1 + r_2)\tfrac{\sqrt{13}}{3}'
+++

A frustum of a cone on the z axis: its base a disk of radius 4 at z = 0, its top a
disk of radius 2 at z = 3, its side the cone between their rims. Shrink it by 1: the solid of
every point at least 1 from its outside.

The base moves up to z = 1 and the top down to z = 2, and the side moves in along its normal by 1,
to the cone of the same angle whose radius at height z is 4 − 2z/3 − √13/3 (a step in along
the normal of 1 is a step of √13/3 across). The result is a smaller frustum of the same cone, from
radius (10 − √13)/3 ≈ 2.13 at z = 1 to radius (8 − √13)/3 ≈ 1.46 at z = 2. A solid that is
convex and shrunk keeps every edge sharp, so the kind of join makes no difference.

**As a set:**

$$\{\,\rho \le 4 - \tfrac{2z}{3},\ 0 \le z \le 3\,\} \ominus B_{1}$$
