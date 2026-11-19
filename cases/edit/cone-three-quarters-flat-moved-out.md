+++
title = "Three quarters of a cone, one flat moved out past its axis"
tags = ["edit", "move-face", "cone", "apex", "concave", "analytic"]
origin = "OCCT tests/offset/faces_type_i/B3"

[volume]
value = 1439.2106966197196
exact = "πR²H/3 − (H/2R)(R³·acos(d/R)/3 − 2dRw/3 + d³·ln((R + w)/d)/3), R = 10, H = 15, d = 4, w = √(R² − d²) = √84: the cone less half its part beyond the plane y = −d, which is (H/R)∫ from d to R of the segment r²·acos(d/r) − d√(r² − d²) of each slice of radius r"
tex = '\tfrac{\pi R^2 H}{3} - \tfrac{H}{2R}\left(\tfrac{R^3}{3}\arccos\tfrac{d}{R} - \tfrac{2}{3}dRw + \tfrac{d^3}{3}\ln\tfrac{R+w}{d}\right),\quad R = 10,\ H = 15,\ d = 4,\ w = \sqrt{R^2 - d^2}'

[area]
value = 846.370691664868
exact = "πRL + πR² − (1 + L/R)·(R²·acos(d/R) − dw)/2 + (H/R)(R − d)²/2 + (H/R)(Rw − d²·ln((R + w)/d))/2, L = √(R² + H²): the cone's side (L/R times the area beneath it) and its base, each less half the segment beyond y = −d; the flat left in x = 0, a triangle; and the moved flat in y = −d, half a hyperbolic section"
tex = '\pi R L + \pi R^2 - \left(1 + \tfrac{L}{R}\right)\tfrac{R^2\arccos(d/R) - dw}{2} + \tfrac{H}{R}\cdot\tfrac{(R-d)^2}{2} + \tfrac{H}{R}\cdot\tfrac{Rw - d^2\ln\frac{R+w}{d}}{2},\quad L = \sqrt{R^2 + H^2}'
+++

Three quarters of the cone whose base, of radius 10, lies in the plane z = 0 centered on the
origin and whose apex is (0, 0, 15): the part whose angle about the z axis, φ, measured from +x
toward +y, lies between 0° and 270°. Its faces are the cone; the base, three quarters of a disk;
and two flat triangles through the axis, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the
half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the axis, from the base's center to the apex,
in a concave edge.

Then edit the solid directly: move the flat at φ = 0° by 4 outward along its normal, −y, to the
plane y = −4. The faces beside it follow it: the cone and the base run on round into the quarter
that was missing, as far as the moved flat, and the other flat, in x = 0, is trimmed back to
y ≤ −4. The two flats meet along the line x = 0, y = −4, in a concave edge from the base up to the
cone at z = 9. The apex is left inside the conical face.

So the result is the whole cone less its part with x > 0 and y < −4.

**As a set:**

$$\{\, \rho \le 10\left(1 - \tfrac{z}{15}\right),\ 0 \le z \le 15 \,\} \setminus \{\, x > 0,\ y < -4 \,\}$$
