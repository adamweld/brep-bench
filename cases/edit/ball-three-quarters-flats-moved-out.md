+++
title = "Three quarters of a ball, both its flats moved out"
tags = ["edit", "move-face", "sphere", "concave", "analytic"]
origin = "OCCT tests/offset/faces_type_i/D2"

[volume]
value = 503.62215279274966
exact = "4πR³/3 − ∫ from −w to −d of (s²·acos(d/s) − d√(s² − d²)) dy, R = 5, d = 2, w = √(R² − d²), s = √(R² − y²): the ball less its part with x > 2, y < −2, whose slice at y is the segment of the disk of radius s beyond x = 2"
tex = '\tfrac{4\pi R^3}{3} - \int_{-w}^{-d}\left(s^2\arccos\tfrac{d}{s} - d\sqrt{s^2 - d^2}\right)dy,\quad R = 5,\ d = 2,\ w = \sqrt{R^2 - d^2},\ s = \sqrt{R^2 - y^2}'

[area]
value = 320.4807428941946
exact = "4πR² − ∫ from −w to −d of 2R·acos(d/s) dy + 2(w²·acos(d/w) − d√(w² − d²)): the sphere less its part with x > 2, y < −2 (at each y an arc of length 2s·acos(d/s) on a sphere's zone of slant R/s), and two flats, each the segment of a disk of radius w beyond a chord at distance d"
tex = '4\pi R^2 - \int_{-w}^{-d} 2R\arccos\tfrac{d}{s}\,dy + 2\left(w^2\arccos\tfrac{d}{w} - d\sqrt{w^2 - d^2}\right)'
+++

Three quarters of the ball of radius 5 about the origin: the part whose angle about the z axis, φ,
measured from +x toward +y, lies between 0° and 270°. Its faces are the sphere and two flat
half-disks of radius 5 through the axis, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the
half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the axis, the diameter from (0, 0, −5) to (0,
0, 5), in a concave edge.

Then edit the solid directly: move the flat at φ = 0° by 2 outward along its normal, −y, to the
plane y = −2, and the flat at φ = 270° by 2 outward along its normal, +x, to the plane x = 2. The
faces beside them follow: the sphere runs on round into the quarter that was missing, as far as the
moved flats. The flat in y = −2 is now the part of the section of the ball there with x ≥ 2, and the
flat in x = 2 the part of its section with y ≤ −2; they meet along the line x = 2, y = −2, in a
concave edge from the sphere to the sphere.

So the result is the ball less its part with x > 2 and y < −2.

**As a set:**

$$\{\, x^2 + y^2 + z^2 \le 25 \,\} \setminus \{\, x > 2,\ y < -2 \,\}$$
