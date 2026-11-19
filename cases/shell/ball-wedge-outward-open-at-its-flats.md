+++
title = "Three quarters of a ball shelled outward, open at both its flats"
tags = ["shell", "outward", "sphere", "pole", "concave", "analytic"]
origin = "OCCT tests/offset/faces_type_i/D2"

[volume]
value = 285.88493147667117
exact = "(3/4)(4π/3)((R + t)³ − R³) = 91π, R = 5, t = 1"
tex = '\tfrac34\cdot\tfrac{4\pi}{3}\left((R + t)^3 - R^3\right) = 91\pi,\quad R = 5,\ t = 1'

[area]
value = 609.4689747964198
exact = "(3/4)·4π((R + t)² + R²) + 2·(π/2)((R + t)² − R²) = 183π + 11π = 194π: the two spheres and the two half annuli"
tex = '\tfrac34\cdot 4\pi\left((R + t)^2 + R^2\right) + 2\cdot\tfrac{\pi}{2}\left((R + t)^2 - R^2\right) = 194\pi'
+++

Three quarters of the ball of radius 5 centered on the origin, from pole to pole: the part whose
angle about the z axis, φ, measured from +x toward +y, lies between 0° and 270°. Its poles are on
the z axis, at (0, 0, ±5). Its faces are the sphere and two flat half disks, in the half-plane
y = 0, x ≥ 0 (φ = 0°) and in the half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the z axis
at a concave edge.

Shell it outward to a wall 1 thick, open at both flat faces. The inside of the wall is the
sphere, unmoved, and the outside is the sphere moved 1 outward along its normal, onto the sphere of
radius 6. The moved face runs on until it meets the surfaces of the openings, the two half-planes,
which are left as rims between the outside and the inside.

So the result is the three quarters of the spherical shell between radii 5 and 6 with φ between
0° and 270°, its rims the two half annuli between those radii in the two half-planes. The rims
meet each other along the z axis from (0, 0, 5) to (0, 0, 6) and from (0, 0, −5) to (0, 0, −6),
through the poles of both spheres.

**As a set:**

$$\{\, 5 \le \lVert p \rVert \le 6,\ 0^\circ \le \varphi \le 270^\circ \,\}$$
