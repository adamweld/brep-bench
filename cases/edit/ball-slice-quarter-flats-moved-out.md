+++
title = "A quarter of a slice of a ball, both its side flats moved out"
tags = ["edit", "move-face", "sphere", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/B6, OCCT tests/offset/faces_type_i/B8"

[volume]
value = 799.85425275454664
exact = "πP/2 − K + 2d²h, R = 10, h = 6, d = 3, P = R²h − h³/3, K = h(R² − h²/3)(π/4 − atan(d/q)) + (2R³/3)·atan(hd/(Rq)) − (d/3)(3R² − d²)·asin(h/c) − 2dhq/3 + d²h, c = √(R² − d²), q = √(R² − d² − h²): the slice, ∫ from 0 to h of π(R² − z²), less the two segments beyond x = −d and y = −d, plus the corner they share (K, the integral of the corner x, y ≥ d of each circle)"
tex = '\tfrac{\pi P}{2} - K + 2d^2h,\quad P = R^2 h - \tfrac{h^3}{3},\ R = 10,\ h = 6,\ d = 3'

[area]
value = 528.70832925003884
exact = "R(πh/2 + 2J) + Ω(√(R² − h²)) + Ω(R) + hq + c²·asin(h/c) + 2dh, J = h·atan(d/q) + d·asin(h/c) − R·atan(hd/(Rq)), Ω(s) = πs² − 2g(s) + D(s), g(s) = s²·acos(d/s) − d√(s² − d²), D(s) = s²(π/4 − asin(d/s)) − d√(s² − d²) + d²: the sphere (R dz times the angle of each circle's arc left, π/2 + 2·asin(d/s)), the top and the bottom (the disk less its two segments plus their shared corner), and the two flats, each the part of its plane between −d and √(s² − d²)"
tex = 'R\left(\tfrac{\pi h}{2} + 2J\right) + \Omega\!\left(\sqrt{R^2 - h^2}\right) + \Omega(R) + hq + c^2\arcsin\tfrac{h}{c} + 2dh,\quad \Omega(s) = \pi s^2 - 2g(s) + D(s)'
+++

The ball of radius 10 centered on the origin, sliced by the planes z = 0 and z = 6, and a quarter
of the slice: the part with x ≥ 0 and y ≥ 0, whose angle φ about the z axis lies between 0° and
90°. Its faces are the sphere; a flat bottom, a quarter disk of radius 10 in the plane z = 0; a flat
top, a quarter disk of radius 8 in the plane z = 6; and two side flats, in the half-plane y = 0,
x ≥ 0 and in the half-plane x = 0, y ≥ 0, which meet along the z axis from z = 0 to z = 6 in a
convex edge.

Then edit the solid directly: move each side flat 3 outward along its normal, the first to the
plane y = −3 and the second to the plane x = −3. The faces beside them follow: the sphere, the top
and the bottom run on round past the old edge to meet both moved flats, and the two flats are
extended until they meet each other, along the line x = y = −3 from z = 0 to z = 6.

So the result is the part of the slice with x ≥ −3 and y ≥ −3.

**As a set:**

$$B_{10} \cap \{\, x \ge -3,\ y \ge -3,\ 0 \le z \le 6 \,\}$$
