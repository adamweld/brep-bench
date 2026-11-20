+++
title = "A prism with a half-round bump, one side drafted, with a ball sunk in its top"
tags = ["draft", "planar", "cylinder", "sphere", "boolean", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug422_2"

[volume]
value = 2808.5403655223976
exact = "2400 + 600t + 80π + 36π − 8π/3, t = tan 5°: the prism's section at height z is the strip of width w = 20 + (10 − z)t, 12 across, with the half disk of radius 4 (8π) on its far side, so 12(200 + 50t) + 80π; the ball of radius 3 adds all but the cap of height 1 under the top, 36π − 8π/3"
tex = '2400 + 600t + 80\pi + 36\pi - \tfrac{8\pi}{3},\quad t = \tan 5^\circ'

[area]
value = 1314.1748914216782
exact = "top 240 + 8π − 5π (the disk of radius √5 the ball cuts out), bottom 12(20 + 10t) + 8π, the flat y = 0: 200 + 50t, the flats at y = 12: 200 + 50t − 80, the half round 40π, the side x = 0: 120, the drafted side 120/cos 5°, the ball's own 36π − 6π = 30π"
tex = '(240 + 3\pi) + (12(20 + 10t) + 8\pi) + (200 + 50t) + (120 + 50t) + 40\pi + 120 + \tfrac{120}{\cos 5^\circ} + 30\pi'
+++

A tall prism whose outline is a rectangle with a half disk on one side: the rectangle 0 ≤ x ≤ 20,
0 ≤ y ≤ 12 in the plane z = 0, and on its side y = 12 the half disk of radius 4 about (10, 12), on
the side y ≥ 12, extruded to z = 10. Its sides are the plane x = 0, the plane y = 0, the plane
y = 12 (in two flat pieces, 0 ≤ x ≤ 6 and 14 ≤ x ≤ 20, either side of the half round), the half
cylinder of the half disk, and the plane x = 20.

Draft the side x = 20 by 5°, with the pull direction +z and the neutral plane z = 10, the top: the
face turns about its top edge until it leans 5° from the z direction, inward as it rises, so that at
height z it is the plane x = 20 + (10 − z)·tan 5°. No other face is drafted: the planes y = 0 and
y = 12 and the bottom run on to meet it.

Fuse on a ball of radius 3 centred at (6, 4, 12), above the top: it crosses the plane z = 10 in the
circle of radius √5 about (6, 4), wholly within the top face, and its lowest point, z = 9, is inside
the prism. The solid is the prism and the ball together.

**As a set:**

$$\Bigl(\{\,0 \le x \le 20 + (10-z)\tan 5^\circ,\ 0 \le y \le 12,\ 0 \le z \le 10\,\} \cup \{\,(x-10)^2 + (y-12)^2 \le 16,\ y \ge 12,\ 0 \le x \le 20 + (10-z)\tan 5^\circ,\ 0 \le z \le 10\,\}\Bigr) \cup \{\,(x-6)^2 + (y-4)^2 + (z-12)^2 \le 9\,\}$$
