+++
title = "Two equal posts a radius apart, united"
tags = ["boolean", "union", "analytic", "cylinder", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/ZF2"

[volume]
value = 1263.7039021427074
exact = "V = 10·(2πr² − Λ), r = 5, Λ = 2r²·arccos(1/2) − (r/2)·√3r = 50π/3 − 25√3/2: the two discs less their lens"
tex = '10\,\bigl(2\pi r^2 - \Lambda\bigr),\quad \Lambda = \tfrac{50\pi}{3} - \tfrac{25\sqrt3}{2},\ r = 5'

[area]
value = 671.6198009071805
exact = "area = 2(2πr² − Λ) + 10·P, P = 2·(2πr − 2r·π/3) = 40π/3: the footprint twice, and the outline (each circle less the arc of 2π/3 inside the other) over the height"
tex = '2\,(2\pi r^2 - \Lambda) + 10\cdot\tfrac{40\pi}{3},\quad \Lambda = \tfrac{50\pi}{3} - \tfrac{25\sqrt3}{2},\ r = 5'
+++

Two cylinders of radius 5 and height 10, both standing on the xy plane from z = 0 to z = 10: the first with its axis the z axis, the second with its axis the vertical line through (5, 0), a radius away. Make their union.

Their discs overlap in a lens, each circle passing through the other's centre. The two tops lie in one plane, and so do the two bottoms; the walls cross along two vertical lines, at (2.5, ±2.5√3).

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \{\,(x-5)^2 + y^2 \le 5^2,\ 0 \le z \le 10\,\}$$
