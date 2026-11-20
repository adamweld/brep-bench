+++
title = "A slab of a horn torus, both its flats moved out"
tags = ["edit", "move-face", "torus", "pole", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/D6"

[volume]
value = 2210.574159152956
exact = "4πR(h·w + r²·asin(h/r)) = 20π(12 + 25·asin(4/5)), R = r = 5, h = 4, w = √(r² − h²) = 3: ∫ from −h to h of each slice, an annulus of area 4πR√(r² − z²)"
tex = '4\pi R\left(h w + r^2\arcsin\tfrac{h}{r}\right) = 20\pi\left(12 + 25\arcsin\tfrac{4}{5}\right),\quad R = r = 5,\ h = 4,\ w = 3'

[area]
value = 959.6278873473374
exact = "8πrR·asin(h/r) + 8πRw = 200π·asin(4/5) + 120π: the torus between z = −4 and 4, by Pappus (the cos terms cancel), and two annuli from R − w to R + w"
tex = '8\pi r R\arcsin\tfrac{h}{r} + 8\pi R w = 200\pi\arcsin\tfrac{4}{5} + 120\pi'
+++

A horn torus: the solid swept by the disk of radius 5 centered on (5, 0, 0) in the plane y = 0,
turned a full turn about the z axis. Its tube touches the axis at the origin, where its surface
pinches to a point. Take the slab of it between z = −3 and z = 3. Its faces are the torus, which
meets the axis at the origin, and two flat annuli between radii 1 and 9, in the planes z = −3 and z
= 3.

Then edit the solid directly: move each flat 1 outward along its normal, the upper one up to the
plane z = 4 and the lower one down to z = −4. The torus runs on to meet them, so each flat is now
the annulus between radii 2 and 8.

So the result is the horn torus between z = −4 and z = 4.

**As a set:**

$$\{\, (\rho - 5)^2 + z^2 \le 25,\ |z| \le 4 \,\}$$
