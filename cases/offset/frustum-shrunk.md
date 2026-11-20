+++
title = "A frustum of very different radii shrunk by 1"
tags = ["offset", "shrink", "cone", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug5805_49"

[volume]
value = 265.75546743700325
exact = "(πh/3)(R² + Rr + r²), h = 4, R = 7, r = 5/3: the frustum between z = 1 and z = 5 whose cone is the side moved 1 inward, 2284π/27"
tex = '\tfrac{\pi h}{3}\left(R^2 + Rr + r^2\right) = \tfrac{2284\pi}{27},\quad h = 4,\ R = 7,\ r = \tfrac{5}{3}'

[area]
value = 344.17892849328179
exact = "πR² + πr² + π(R + r)s = 49π + 25π/9 + (26π/3)(20/3) = 986π/9, s = 20/3 the slant"
tex = '\pi R^2 + \pi r^2 + \pi (R + r) s = \tfrac{986\pi}{9},\quad s = \tfrac{20}{3}'
+++

A cone frustum on the z axis, shrunk by 1: the solid of every point at least 1 from its outside.
The frustum has radius 10 at its base, z = 0, and radius 2 at its top, z = 6, so its side is the
cone ρ = 10 − 4z/3, leaning α from the z direction with tan α = 4/3.

Each face moves 1 inward and the edges stay sharp. The base and the top become the planes z = 1
and z = 5. The side, moved 1 inward along its normal, is the cone of the same angle whose radius is
less by 1/cos α = 5/3: ρ = 25/3 − 4z/3. The shrunk frustum is the one between z = 1 and z = 5 under
that cone, of radius 7 at z = 1 and 5/3 at z = 5.

**As a set:**

$$\{\, \rho \le 10 - \tfrac{4z}{3},\ 0 \le z \le 6 \,\} \ominus B_{1}$$
