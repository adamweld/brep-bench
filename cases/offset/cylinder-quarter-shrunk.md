+++
title = "A quarter of a cylinder shrunk by 2"
tags = ["offset", "shrink", "cylinder", "planar", "analytic"]
origin = "OCCT tests/offset/shape_type_i/F2"

[volume]
value = 90.40805097405597
exact = "D(s)·(h − 2d), s = 8, d = 2, h = 8, D(s) = s²(π/4 − asin(d/s)) − d√(s² − d²) + d²: the area of the disk of radius s with x ≥ d and y ≥ d"
tex = 'D(s)\,(h - 2d),\quad s = 8,\ d = 2,\ h = 8'

[area]
value = 125.26570515469031
exact = "2D(s) + (h − 2d)(s(π/2 − 2·asin(d/s)) + 2(√(s² − d²) − d)): the top and bottom, the wall's arc, and the two flats, each running from d to √(s² − d²)"
tex = '2D(s) + (h - 2d)\Bigl(s\bigl(\tfrac{\pi}{2} - 2\arcsin\tfrac{d}{s}\bigr) + 2\bigl(\sqrt{s^2 - d^2} - d\bigr)\Bigr)'
+++

A quarter of a cylinder: the part of the cylinder of radius 10 about the z axis, from z = 0 to z =
8, with x ≥ 0 and y ≥ 0 (φ from 0° to 90°). Its faces are the curved wall, a flat bottom and a flat
top, each a quarter disk, and two flat cut faces, in the half-plane y = 0, x ≥ 0 and in the half-
plane x = 0, y ≥ 0, which meet along the z axis in a convex edge.

Shrink it by 2: the solid of every point at least 2 from the quarter's outside.

The wall moves in to the radius 8, the bottom up to z = 2, the top down to z = 6, and the cut faces
in to the planes y = 2 and x = 2. The result is the part of the cylinder of radius 8 from z = 2 to z
= 6 with x ≥ 2 and y ≥ 2.

**As a set:**

$$\{\, \rho \le 8,\ x \ge 2,\ y \ge 2,\ 2 \le z \le 6 \,\}$$
