+++
title = "Three quarters of a slice of a ball, both its side flats moved out"
tags = ["edit", "move-face", "sphere", "planar", "concave", "analytic"]
origin = "OCCT tests/offset/faces_type_i/C1, OCCT tests/offset/faces_type_i/C3, OCCT tests/offset/faces_type_i/C9"

[volume]
value = 1443.0427011174712
exact = "πP − K, R = 10, h = 6, d = 2, P = R²h − h³/3, K = h(R² − h²/3)(π/4 − atan(d/q)) + (2R³/3)·atan(hd/(Rq)) − (d/3)(3R² − d²)·asin(h/c) − 2dhq/3 + d²h, c = √(R² − d²), q = √(R² − d² − h²): the slice, ∫ from 0 to h of π(R² − z²), less K, the integral of the corner x, y ≥ d of each circle"
tex = '\pi P - K,\quad P = R^2 h - \tfrac{h^3}{3},\ R = 10,\ h = 6,\ d = 2'

[area]
value = 844.24423348532689
exact = "R(3πh/2 + 2J) + π(R² − h²) − D(√(R² − h²)) + πR² − D(R) + hq + c²·asin(h/c) − 2dh, J = h·atan(d/q) + d·asin(h/c) − R·atan(hd/(Rq)), D(s) = s²(π/4 − asin(d/s)) − d√(s² − d²) + d²: the sphere (R dz times 3π/2 + 2·asin(d/s), the arc of each circle left), the top and the bottom (the whole disk less its corner beyond both flats), and the two flats, each the part of its plane from d to √(s² − d²)"
tex = 'R\left(\tfrac{3\pi h}{2} + 2J\right) + \pi(R^2 - h^2) - D\!\left(\sqrt{R^2 - h^2}\right) + \pi R^2 - D(R) + hq + c^2\arcsin\tfrac{h}{c} - 2dh'
+++

The ball of radius 10 centered on the origin, sliced by the planes z = 0 and z = 6, and three
quarters of the slice: the part whose angle φ about the z axis, measured from +x toward +y, lies
between 0° and 270°. Its faces are the sphere; a flat bottom, three quarters of a disk of radius 10
in the plane z = 0; a flat top, three quarters of a disk of radius 8 in the plane z = 6; and two
side flats, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the half-plane x = 0, y ≤ 0 (φ = 270°),
which meet along the z axis from z = 0 to z = 6 in a concave edge.

Then edit the solid directly: move each side flat 2 outward along its normal, the first, at φ = 0°,
to the plane y = −2 and the second, at φ = 270°, to the plane x = 2. The faces beside them follow:
the sphere, the top and the bottom run on round into the quarter that was missing, as far as the
moved flats, and the two flats are trimmed to meet each other, along the line x = 2, y = −2 from
z = 0 to z = 6, in a concave edge.

So the result is the slice less its part with x > 2 and y < −2.

**As a set:**

$$\bigl(B_{10} \cap \{\, 0 \le z \le 6 \,\}\bigr) \setminus \{\, x > 2,\ y < -2 \,\}$$
