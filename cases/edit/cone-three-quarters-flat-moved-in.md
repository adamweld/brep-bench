+++
title = "Three quarters of a cone, one flat moved in"
tags = ["edit", "move-face", "cone", "apex", "concave", "analytic"]
origin = "OCCT tests/offset/faces_type_i/B4"

[volume]
value = 916.98379357262544
exact = "πR²H/6 + (H/2R)·S, R = 10, H = 15, d = 4, w = √(R² − d²) = √84, S = R³·acos(d/R)/3 − 2dRw/3 + d³·ln((R + w)/d)/3: half the cone plus half its part beyond the plane y = d (the cone less its part with x > 0 and y < d)"
tex = '\tfrac{\pi R^2 H}{6} + \tfrac{H}{2R}S,\quad S = \tfrac{R^3}{3}\arccos\tfrac{d}{R} - \tfrac{2}{3}dRw + \tfrac{d^3}{3}\ln\tfrac{R+w}{d},\quad R = 10,\ H = 15,\ d = 4'

[area]
value = 724.28030047036171
exact = "(1 + L/R)·Ω + F₁ + F₂, L = √(R² + H²), Ω = (πR² + R²·acos(d/R) − dw)/2, F₁ = H(w/2 − (d²/(2R))·ln((R + w)/d)), F₂ = HR/2 + H(d − d²/(2R)): the cone's side and base over the footprint Ω; the moved flat in y = d, ∫ from x = 0 to w of the cone's height H(1 − √(x² + d²)/R); and the flat in x = 0, the triangle under the cone from y = −R to 0 plus the part from y = 0 to d"
tex = '\left(1 + \tfrac{L}{R}\right)\Omega + F_1 + F_2,\quad \Omega = \tfrac{\pi R^2 + R^2\arccos(d/R) - dw}{2},\ F_1 = H\left(\tfrac{w}{2} - \tfrac{d^2}{2R}\ln\tfrac{R+w}{d}\right),\ F_2 = \tfrac{HR}{2} + H\left(d - \tfrac{d^2}{2R}\right)'
+++

Three quarters of the cone whose base, of radius 10, lies in the plane z = 0 centered on the
origin and whose apex is (0, 0, 15): the part whose angle about the z axis, φ, measured from +x
toward +y, lies between 0° and 270°. Its faces are the cone; the base, three quarters of a disk; and
two flat triangles through the axis, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the half-plane
x = 0, y ≤ 0 (φ = 270°), which meet along the axis, from the base's center to the apex, in a
concave edge.

Then edit the solid directly: move the flat at φ = 0° by 4 inward along its normal, +y, to the plane
y = 4. The faces beside it follow it: the cone and the base are trimmed back to y ≥ 4 where x > 0,
and the other flat, in x = 0, is extended on up to y = 4. The two flats meet along the line
x = 0, y = 4, in a concave edge from the base up to the cone at z = 9. The apex stays, a corner of
the flat in x = 0.

So the result is the cone less its part with x > 0 and y < 4.

**As a set:**

$$\{\, \rho \le 10\left(1 - \tfrac{z}{15}\right),\ 0 \le z \le 15 \,\} \setminus \{\, x > 0,\ y < 4 \,\}$$
