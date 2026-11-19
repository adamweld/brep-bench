+++
title = "A ridge nearly flat, rounded by a radius of 180"
tags = ["fillet", "prism", "planar", "cylinder", "near-degenerate"]
origin = "brep-conformance 1.0.0, fillets/nearly-flat-edge"

[volume]
value = 240.7947137228925
exact = "6(40 + 16 tan α − r²(tan α − α)), α = 0.5°, r = 180"
tex = '6\left(40 + 16\tan\alpha - r^2(\tan\alpha - \alpha)\right),\quad \alpha = 0.5^\circ,\ r = 180'

[area]
value = 236.26625383031265
exact = "2P + 6(18 + 8/cos α − 2r tan α + 2rα), P = 40 + 16 tan α − r²(tan α − α), α = 0.5°, r = 180"
tex = '2P + 6\left(18 + \tfrac{8}{\cos\alpha} - 2r\tan\alpha + 2r\alpha\right),\quad P = 40 + 16\tan\alpha - r^2(\tan\alpha - \alpha),\ \alpha = 0.5^\circ,\ r = 180'
+++

A prism 6 high, from z = 0 to z = 6, on the pentagon (0, 0), (8, 0), (8, 5), (4, 5 + 4 tan 0.5°),
(0, 5): a block 8 by 5 whose top is two faces each tilted 0.5° up to a ridge along x = 4, so they
meet at 179°, nearly flat. Round the ridge with a fillet of radius 180.

A ball of radius 180 sits on the two roof faces touching each 180 tan 0.5° ≈ 1.571 from the ridge,
inside the 4 each face has, and the round between those lines is an arc of a cylinder of radius
180 turning through 1°. The round's radius is twenty times the solid's size, and the corner it
replaces is only 4 tan 0.5° ≈ 0.035 high.

**As a set:**

$$\Bigl(\mathrm{poly}\bigl((0, 0), (8, 0), (8, 5), (4, 5 + 4\tan 0.5^\circ), (0, 5)\bigr) \setminus \bigl(\{\, |x - 4| < 180\sin 0.5^\circ \,\} \setminus \{\, (x - 4)^2 + (y - y_c)^2 \le 180^2 \,\}\bigr)\Bigr) \times [0, 6],\quad y_c = 5 + 4\tan 0.5^\circ - \tfrac{180}{\cos 0.5^\circ}$$
