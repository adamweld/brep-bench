+++
title = "A quarter of the upper half of a horn torus, its flat moved up past the pinch"
tags = ["edit", "move-face", "torus", "pole", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/K1"

[volume]
value = 87.82621067873188
exact = "πR(r²π/4 − (d·w + r²·asin(d/r))/2) = 5π(25π/4 − (12 + 25·asin(3/5))/2), R = r = 5, d = 3, w = √(r² − d²) = 4: a quarter of ∫ from d to r of each slice, an annulus of area 4πRw"
tex = '\pi R\left(\tfrac{\pi r^2}{4} - \tfrac{dw + r^2\arcsin\frac{d}{r}}{2}\right) = 5\pi\left(\tfrac{25\pi}{4} - \tfrac{12 + 25\arcsin\frac{3}{5}}{2}\right),\quad R = r = 5,\ d = 3,\ w = 4'

[area]
value = 158.02621008644675
exact = "(25π/2)(π − 2ψ) + 20π + 2(25·acos(3/5) − 12), ψ = asin(3/5): the torus above z = 3 (a quarter turn of its arc, whose moment about the axis is R times its length, as the cos terms cancel), the quarter annulus between radii 1 and 9 in z = 3, and the two circular segments of the tube above the chord z = 3"
tex = '\tfrac{25\pi}{2}(\pi - 2\psi) + 20\pi + 2\left(25\arccos\tfrac{3}{5} - 12\right),\quad \psi = \arcsin\tfrac{3}{5}'
+++

A horn torus: the solid swept by the disk of radius 5 centered on (5, 0, 0) in the plane y = 0,
turned about the z axis. Its tube touches the axis at the origin, where its surface pinches to a
point. Take its upper half, z ≥ 0, and of that the quarter turn from φ = 0° to φ = 90°, where φ is
the angle about the z axis from +x toward +y. Its faces are the torus; a flat quarter disk of radius
10 in the plane z = 0, whose corner is the pinch; and two flat half disks of radius 5 above z = 0,
one in the plane y = 0 centered on (5, 0, 0) and one in the plane x = 0 centered on (0, 5, 0),
which touch at the pinch.

Then edit the solid directly: move the quarter disk in z = 0 up 3, against its outward normal −z,
to the plane z = 3. The torus is trimmed there, so the flat becomes the quarter of the annulus
between radii 1 and 9 in the plane z = 3, and the two half disks are trimmed to the circular
segments of their disks above z = 3. The pinch is no longer on the solid.

So the result is the quarter, φ from 0° to 90°, of the horn torus above z = 3.

**As a set:**

$$\{\, (\rho - 5)^2 + z^2 \le 25,\ z \ge 3,\ x \ge 0,\ y \ge 0 \,\}$$
