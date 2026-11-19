+++
title = "A quarter of a slice of a ball, both its side flats moved in"
tags = ["edit", "move-face", "sphere", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/B9"

[volume]
value = 215.71821997793958
exact = "h(R² − h²/3)(π/4 − atan(d/q)) + (2R³/3)·atan(hd/(Rq)) − (d/3)(3R² − d²)·asin(h/c) − 2dhq/3 + d²h, R = 10, h = 6, d = 2, c = √(R² − d²) = √96, q = √(R² − d² − h²) = √60: ∫ from 0 to h of the corner x, y ≥ d of the slice, the disk of radius √(R² − z²)"
tex = 'h\left(R^2 - \tfrac{h^2}{3}\right)\left(\tfrac{\pi}{4} - \arctan\tfrac{d}{q}\right) + \tfrac{2R^3}{3}\arctan\tfrac{hd}{Rq} - \tfrac{d}{3}\left(3R^2 - d^2\right)\arcsin\tfrac{h}{c} - \tfrac{2}{3}dhq + d^2h,\quad R = 10,\ h = 6,\ d = 2,\ c = \sqrt{R^2 - d^2},\ q = \sqrt{R^2 - d^2 - h^2}'

[area]
value = 219.4588233218229
exact = "R(πh/2 − 2(h·atan(d/q) + d·asin(h/c) − R·atan(hd/(Rq)))) + D(√(R² − h²)) + D(R) + hq + c²·asin(h/c) − 2dh, D(s) = s²(π/4 − asin(d/s)) − d√(s² − d²) + d²: the sphere (R dz times the angle of each circle in the corner), the top and the bottom (the corner of a disk of radius s), and the two side flats"
tex = 'R\left(\tfrac{\pi h}{2} - 2\left(h\arctan\tfrac{d}{q} + d\arcsin\tfrac{h}{c} - R\arctan\tfrac{hd}{Rq}\right)\right) + D\!\left(\sqrt{R^2 - h^2}\right) + D(R) + hq + c^2\arcsin\tfrac{h}{c} - 2dh,\quad D(s) = s^2\left(\tfrac{\pi}{4} - \arcsin\tfrac{d}{s}\right) - d\sqrt{s^2 - d^2} + d^2'
+++

The ball of radius 10 centered on the origin, sliced by the planes z = 0 and z = 6, and a quarter
of the slice: the part with x ≥ 0 and y ≥ 0, whose angle φ about the z axis lies between 0° and
90°. Its faces are the sphere; a flat bottom, a quarter disk of radius 10 in the plane z = 0; a flat
top, a quarter disk of radius 8 in the plane z = 6; and two side flats, in the half-plane y = 0,
x ≥ 0 and in the half-plane x = 0, y ≥ 0, which meet along the z axis from z = 0 to z = 6.

Then edit the solid directly: move both side flats 2 inward, along their inward normals, the first
to the plane y = 2 and the second to the plane x = 2. The faces beside them follow: the sphere,
the top and the bottom are trimmed back to meet them, and the two flats are trimmed to meet each
other along the line x = y = 2, from z = 0 to z = 6.

So the result is the part of the slice with x ≥ 2 and y ≥ 2.

**As a set:**

$$B_{10} \cap \{\, x \ge 2,\ y \ge 2,\ 0 \le z \le 6 \,\}$$
