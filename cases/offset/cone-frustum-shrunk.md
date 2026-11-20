+++
title = "A cone frustum with very different radii, shrunk by 1"
tags = ["offset", "shrink", "cone", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug5805_29"

[volume]
value = 178.33184988050858
exact = "(4π/3)(r₁² + r₁r₂ + r₂²), r₁ = 7 − √2, r₂ = 3 − √2: the frustum from z = 1 to z = 5 inside the cone moved in by 1"
tex = '\tfrac{4\pi}{3}\left(r_1^2 + r_1 r_2 + r_2^2\right),\quad r_1 = 7 - \sqrt 2,\ r_2 = 3 - \sqrt 2'

[area]
value = 233.37092082829780
exact = "π(r₁² + r₂²) + π(r₁ + r₂)·4√2: the bottom, the top and the side, of slant length 4√2"
tex = '\pi\left(r_1^2 + r_2^2\right) + \pi(r_1 + r_2)\cdot 4\sqrt 2'
+++

A cone frustum on the z axis: the disk of radius 8 in the plane z = 0 and the disk of radius 2 in
the plane z = 6, joined by the cone ρ = 8 − z through their rims, whose side leans 45° from the z
axis.

Shrink it by 1: the solid of every point at least 1 from its outside. The ends move in to z = 1
and z = 5, and the side moves in along its normal onto the cone 1 inside it, which is the same
cone with its radius less by 1/cos 45° = √2: ρ = 8 − z − √2. All the edges are convex, so they
stay sharp. The result is the frustum of radius 7 − √2 at z = 1 and 3 − √2 at z = 5.

**As a set:**

$$\{\,\rho \le 8 - z,\ 0 \le z \le 6\,\} \ominus B_{1}$$
