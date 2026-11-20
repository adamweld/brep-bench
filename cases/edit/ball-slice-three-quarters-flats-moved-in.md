+++
title = "Three quarters of a slice of a ball, both its side flats moved in"
tags = ["edit", "move-face", "sphere", "planar", "concave", "analytic"]
origin = "OCCT tests/offset/faces_type_i/C2, OCCT tests/offset/faces_type_i/C4"

[volume]
value = 858.90666834086419
exact = "πP/2 + K − 2d²h, R = 10, h = 6, d = 3, P = R²h − h³/3, K = h(R² − h²/3)(π/4 − atan(d/q)) + (2R³/3)·atan(hd/(Rq)) − (d/3)(3R² − d²)·asin(h/c) − 2dhq/3 + d²h, c = √(R² − d²), q = √(R² − d² − h²): the slice's part with x ≤ −d or y ≥ d, two segments (each the integral of s²·acos(d/s) − d√(s² − d²) over the slice) less the corner they share, K"
tex = '\tfrac{\pi P}{2} + K - 2d^2h,\quad P = R^2 h - \tfrac{h^3}{3},\ R = 10,\ h = 6,\ d = 3'

[area]
value = 648.30000449890262
exact = "R(3πh/2 − 2J) + Σ(√(R² − h²)) + Σ(R) + hq + c²·asin(h/c) + 2dh, J = h·atan(d/q) + d·asin(h/c) − R·atan(hd/(Rq)), Σ(s) = 2g(s) − D(s), g(s) = s²·acos(d/s) − d√(s² − d²), D(s) = s²(π/4 − asin(d/s)) − d√(s² − d²) + d²: the sphere (R dz times 3π/2 − 2·asin(d/s), the arc of each circle left), the top and the bottom (two segments less their shared corner), and the two flats, each the part of its plane from −d to √(s² − d²)"
tex = 'R\left(\tfrac{3\pi h}{2} - 2J\right) + \Sigma\!\left(\sqrt{R^2 - h^2}\right) + \Sigma(R) + hq + c^2\arcsin\tfrac{h}{c} + 2dh,\quad \Sigma(s) = 2g(s) - D(s)'
+++

The ball of radius 10 centered on the origin, sliced by the planes z = 0 and z = 6, and three
quarters of the slice: the part whose angle φ about the z axis, measured from +x toward +y, lies
between 0° and 270°. Its faces are the sphere; a flat bottom, three quarters of a disk of radius 10
in the plane z = 0; a flat top, three quarters of a disk of radius 8 in the plane z = 6; and two
side flats, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the half-plane x = 0, y ≤ 0 (φ = 270°),
which meet along the z axis from z = 0 to z = 6 in a concave edge.

Then edit the solid directly: move each side flat 3 inward along its inward normal, the first, at
φ = 0°, to the plane y = 3 and the second, at φ = 270°, to the plane x = −3. The faces beside them
follow: the sphere, the top and the bottom are trimmed back to meet the moved flats, and the two
flats are extended until they meet each other, along the line x = −3, y = 3 from z = 0 to z = 6, in
a concave edge.

So the result is the part of the slice with x ≤ −3 or y ≥ 3.

**As a set:**

$$B_{10} \cap \{\, 0 \le z \le 6 \,\} \cap \bigl(\{\, x \le -3 \,\} \cup \{\, y \ge 3 \,\}\bigr)$$
