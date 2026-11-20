+++
title = "The upper half of a horn torus, its flat moved up through the pinch"
tags = ["edit", "move-face", "torus", "pole", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/D5"

[volume]
value = 351.30484271492753
exact = "2πR(r²·acos(d/r) − d·w) = 10π(25·acos(3/5) − 12), R = r = 5, d = 3, w = √(r² − d²) = 4: ∫ from d to r of each slice, an annulus of area 4πR√(r² − z²)"
tex = '2\pi R\left(r^2\arccos\tfrac{d}{r} - d w\right) = 10\pi\left(25\arccos\tfrac{3}{5} - 12\right),\quad R = r = 5,\ d = 3,\ w = 4'

[area]
value = 542.6457967454645
exact = "2πrR(π − 2·asin(d/r)) + 4πRw = 50π(π − 2·asin(3/5)) + 80π: the torus above z = 3, by Pappus (its arc's moment about the axis is R times its length, as the cos terms cancel), and the annulus from R − w to R + w"
tex = '2\pi r R\left(\pi - 2\arcsin\tfrac{d}{r}\right) + 4\pi R w = 50\pi\left(\pi - 2\arcsin\tfrac{3}{5}\right) + 80\pi'
+++

A horn torus: the solid swept by the disk of radius 5 centered on (5, 0, 0) in the plane y = 0,
turned a full turn about the z axis. Its tube touches the axis at the origin, where its surface
pinches to a point. Take its upper half, z ≥ 0. Its faces are the torus and a flat disk of radius 10
in the plane z = 0, whose center is the pinch.

Then edit the solid directly: move the flat 3 inward, up against its outward normal, to the plane z
= 3. The torus is trimmed back to z ≥ 3, so the flat becomes the annulus between radii 1 and 9 in
the plane z = 3, and the pinch at the origin, now cut away, is no longer part of the solid.

So the result is the horn torus less its part below z = 3: a ring with a hole through its middle.

**As a set:**

$$\{\, (\rho - 5)^2 + z^2 \le 25,\ z \ge 3 \,\}$$
