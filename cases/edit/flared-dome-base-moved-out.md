+++
title = "A flared dome's base moved out"
tags = ["edit", "move-face", "revolved", "cone", "ellipsoid", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/G5"

[volume]
value = 281.11436485455334
exact = "(2/3)π·3²·2 + (πh/3)(R² + Rr + r²) = 12π + (4π/3)(289/9 + 17 + 9) = 2416π/27: the half spheroid and the cone's frustum, h = 4, R = 17/3, r = 3"
tex = '\tfrac{2}{3}\pi\cdot 3^2\cdot 2 + \tfrac{\pi h}{3}\left(R^2 + Rr + r^2\right) = \tfrac{2416}{27}\pi,\quad h = 4,\ R = \tfrac{17}{3},\ r = 3'

[area]
value = 276.2721809524044
exact = "289π/9 + 104√13π/9 + 9π + (6π/√5)·ln((3 + √5)/(3 − √5)): the base disk of radius 17/3; the cone, π(R + r)ℓ with ℓ = 4√13/3; and the half spheroid, πa² + (πb²/2e)·ln((1 + e)/(1 − e)) with a = 3, b = 2, e = √5/3"
tex = '\tfrac{289}{9}\pi + \tfrac{104\sqrt{13}}{9}\pi + 9\pi + \tfrac{6\pi}{\sqrt 5}\ln\tfrac{3+\sqrt 5}{3-\sqrt 5}'
+++

A solid turned about the z axis. Its profile, in the half-plane x ≥ 0 (ρ = x, the distance from the axis), runs from the axis at (0, 0) along z = 0 out to (5, 0); straight up and in to (3, 3); then along the quarter ellipse (ρ/3)² + ((z − 3)/2)² = 1 up and in to the axis at (0, 5); and back down the axis. Its faces are a flat base disk of radius 5 in z = 0, a conical flare from radius 5 at z = 0 to radius 3 at z = 3, and a dome that is half a spheroid of semi-axes 3 (across) and 2 (up), with a seam.

Then edit the solid directly: move the base 1 outward along its normal, down to the plane z = −1. The flare runs on down along its cone to meet it, so the base becomes a disk of radius 17/3. The dome is unchanged.

So the result is the dome on a flare that runs from radius 17/3 at z = −1 to radius 3 at z = 3, over a flat base disk.

**As a set:**

$$\{\, -1 \le z \le 3,\ \rho \le 5 - \tfrac{2z}{3} \,\} \cup \{\, z \ge 3,\ (\rho/3)^2 + ((z-3)/2)^2 \le 1 \,\}$$
