+++
title = "A torus grown by 1"
tags = ["offset", "grow", "torus", "analytic"]
origin = "OCCT tests/offset/shape_type_i/E2"

[volume]
value = 3158.273408348595
exact = "2π²R(r + d)², R = 10, r = 3, d = 1"
tex = '2\pi^2 R (r + d)^2,\quad R = 10,\ r = 3,\ d = 1'

[area]
value = 1579.1367041742974
exact = "4π²R(r + d)"
tex = '4\pi^2 R (r + d)'
+++

A solid torus about the z axis, centered on the origin, whose tube has radius 3 about a middle
circle of radius 10. Grow it by 1: the solid of every point within 1 of it.

The grown solid is the torus about the same middle circle whose tube has radius 4.

**As a set:**

$$\{\, (\rho - 10)^2 + z^2 \le 3^2 \,\} \oplus B_1$$
