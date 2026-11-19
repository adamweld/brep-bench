+++
title = "The upper half of a horn torus, its flat moved down past the pinch"
tags = ["edit", "move-face", "torus", "pole", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/D4"

[volume]
value = 2116.096257557412
exact = "πR(πr² + 2dw + 2r²·asin(d/r)) = 5π(25π + 24 + 50·asin(3/5)), R = r = 5, d = 3, w = √(r² − d²) = 4: ∫ from −d to r of each slice, an annulus of area 4πR√(r² − z²)"
tex = '\pi R\left(\pi r^2 + 2dw + 2r^2\arcsin\tfrac{d}{r}\right) = 5\pi\left(25\pi + 24 + 50\arcsin\tfrac{3}{5}\right),\quad R = r = 5,\ d = 3,\ w = 4'

[area]
value = 946.9694679378382
exact = "2πrR(π + 2·asin(d/r)) + 4πRw = 50π(π + 2·asin(3/5)) + 80π: the torus above z = −d, by Pappus (its arc's moment about the axis is R times its length, as the cos terms cancel), and the annulus from R − w to R + w"
tex = '2\pi r R\left(\pi + 2\arcsin\tfrac{d}{r}\right) + 4\pi R w = 50\pi\left(\pi + 2\arcsin\tfrac{3}{5}\right) + 80\pi'
+++

A horn torus: the solid swept by the disk of radius 5 centered on (5, 0, 0) in the plane y = 0,
turned a full turn about the z axis. Its tube touches the axis at the origin, where its surface
pinches to a point. Take its upper half, z ≥ 0. Its faces are the torus and a flat disk of radius
10 in the plane z = 0, whose center is the pinch.

Then edit the solid directly: move the flat 3 down, along its outward normal, to the plane z = −3.
The torus runs on below z = 0 to meet it, so the flat becomes the annulus between radii 1 and 9 in
the plane z = −3, and the pinch at the origin is left inside the toroidal face.

So the result is the horn torus less its part below z = −3.

**As a set:**

$$\{\, (\rho - 5)^2 + z^2 \le 25,\ z \ge -3 \,\}$$
