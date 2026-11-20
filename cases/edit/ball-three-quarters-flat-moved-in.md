+++
title = "Three quarters of a ball, one flat moved in"
tags = ["edit", "move-face", "sphere", "concave", "analytic"]
origin = "OCCT tests/offset/faces_type_i/D1"

[volume]
value = 318.34805556376574
exact = "4πR³/3 − (π/2)(R²(R + d) − (R³ + d³)/3), R = 5, d = 2: the ball less its part with x > 0, y < d, whose slice at each y is half a disk of area π(R² − y²)/2"
tex = '\tfrac{4\pi R^3}{3} - \tfrac{\pi}{2}\left(R^2(R + d) - \tfrac{R^3 + d^3}{3}\right),\quad R = 5,\ d = 2'

[area]
value = 295.9132260575007
exact = "4πR² − πR(R + d) + (πR² − (R²·acos(d/R) − d·w)) + π(R² − d²)/2, w = √(R² − d²) = √21: the sphere less its part with x > 0, y < d (half a zone, πR per unit of y); the flat in x = 0, the disk less its segment beyond y = d; and the moved flat, half a disk of radius w"
tex = '4\pi R^2 - \pi R(R + d) + \left(\pi R^2 - \left(R^2\arccos\tfrac{d}{R} - d w\right)\right) + \tfrac{\pi}{2}(R^2 - d^2),\quad R = 5,\ d = 2,\ w = \sqrt{21}'
+++

Three quarters of the ball of radius 5 about the origin: the part whose angle about the z axis, φ,
measured from +x toward +y, lies between 0° and 270°. Its faces are the sphere and two flat
half-disks of radius 5 through the axis, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the
half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the axis, the diameter from (0, 0, −5) to (0,
0, 5), in a concave edge.

Then edit the solid directly: move the flat at φ = 0° by 2 inward along its normal, +y, to the plane
y = 2. The faces beside it follow it: the sphere is trimmed back to y ≥ 2 where x > 0, and the other
flat, in x = 0, runs on past the axis, from y ≤ 0 to y ≤ 2, to meet it. The two flats meet along the
line x = 0, y = 2, in a concave edge from the sphere to the sphere. The moved flat is the half-disk
of radius √21 in the plane y = 2, x ≥ 0.

So the result is the ball less its part with x > 0 and y < 2.

**As a set:**

$$\{\, x^2 + y^2 + z^2 \le 25 \,\} \setminus \{\, x > 0,\ y < 2 \,\}$$
