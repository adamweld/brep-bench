+++
title = "Three quarters of a cylinder shrunk by 2"
tags = ["offset", "shrink", "cylinder", "planar", "concave", "analytic"]
origin = "OCCT tests/offset/shape_type_i/F3"

[volume]
value = 463.9655400191903
exact = "A·(h − 2d), A = 3πs²/4 − πd²/4 − (d√(s² − d²) + s²·asin(d/s)), s = 8, d = 2, h = 8: three quarters of the disk of radius s less the quarter disk of radius d at the axis, less the two strips between the axis planes and the planes y = d, x = −d"
tex = 'A\,(h - 2d),\quad A = \tfrac{3\pi s^2}{4} - \tfrac{\pi d^2}{4} - d\sqrt{s^2 - d^2} - s^2\arcsin\tfrac{d}{s}'

[area]
value = 441.14178520649006
exact = "2A + (h − 2d)(s(3π/2 − 2·asin(d/s)) + 2√(s² − d²) + πd/2): the top and bottom, the wall's arc, the two flats, and the round"
tex = '2A + (h - 2d)\Bigl(s\bigl(\tfrac{3\pi}{2} - 2\arcsin\tfrac{d}{s}\bigr) + 2\sqrt{s^2 - d^2} + \tfrac{\pi d}{2}\Bigr)'
+++

Three quarters of a cylinder: the part of the cylinder of radius 10 about the z axis, from z = 0 to
z = 8, with φ from 0° to 270° (φ measured from +x toward +y): all but the quarter with x ≥ 0 and y ≤
0. Its faces are the curved wall, a flat bottom and a flat top, each three quarters of a disk, and
two flat cut faces, in the half-plane y = 0, x ≥ 0 and in the half-plane x = 0, y ≤ 0, which meet
along the z axis in a concave edge.

Shrink it by 2: the solid of every point at least 2 from the solid's outside.

The wall moves in to the radius 8, the bottom up to z = 2 and the top down to z = 6. The cut faces
move 2 into the solid, to the plane y = 2 (for x ≥ 0) and the plane x = −2 (for y ≤ 0). At the
concave edge the points within 2 of the missing quarter are removed, so the result is not cornered
there but rounded: the two planes are joined by the quarter of a cylinder of radius 2 about the z
axis, in the quadrant x ≤ 0, y ≥ 0.

**As a set:**

$$\{\, \rho \le 8,\ 2 \le z \le 6,\ d\bigl((x, y),\ \{x \ge 0,\ y \le 0\}\bigr) \ge 2 \,\}$$
