+++
title = "Three quarters of a turned hub, the rim between its cylinder and its cone rounded"
tags = ["fillet", "torus", "cylinder", "cone", "revolve", "analytic"]
origin = "OCCT tests/blend/simple/H6"

[volume]
value = 9983.755554417952
exact = "(3π/2)(6368/3 − m), m = (2t/3)(64 − t/√2) − 16π − 32√2/3, t = 4·tan 22.5° = 4(√2 − 1): by Pappus, the section's moment about the axis, 6368/3, less that of the sliver the round takes off, its kite (two triangles r·t/2 with their vertices' mean ρ) less a 45° sector of radius 4 about (8, 16 − t)"
tex = '\tfrac{3\pi}{2}\left(\tfrac{6368}{3} - m\right),\quad m = \tfrac{2t}{3}\left(64 - \tfrac{t}{\sqrt2}\right) - 16\pi - \tfrac{32\sqrt2}{3},\quad t = 4\tan 22.5^\circ = 4\left(\sqrt2 - 1\right)'

[area]
value = 3642.2556174325296
exact = "(3π/2)(600 + 48√2 + 8π − 24t + t²/(2√2)) + 2(280 − 4t + 2π): by Pappus over the section's edges off the axis, 600 + 40√2 for the polygon's less the lengths t the round takes from the cylinder and the cone, plus 8π + 8√2 for its arc; and the two flat sections, each 280 less the sliver's area 4t − 2π"
tex = '\tfrac{3\pi}{2}\left(600 + 48\sqrt2 + 8\pi - 24t + \tfrac{t^2}{2\sqrt2}\right) + 2\left(280 - 4t + 2\pi\right)'
+++

A hub turned three quarters of a turn: the polygon through (ρ, z) = (0, 0), (20, 0), (20, 6),
(12, 6), (12, 16), (8, 20), (0, 20), drawn in the half-plane y = 0, x ≥ 0 (where ρ = x), turned
about the z axis from φ = 0° to φ = 270°, right-handed. So it is a flange of radius 20, 6 thick;
on it a cylinder of radius 12 up to z = 16; and on that a cone at 45° up to a flat top of radius 8
at z = 20; all cut by the half-planes φ = 0° and φ = 270°, which meet along the z axis in a concave
edge.

Round the convex edge where the cylinder meets the cone, three quarters of the circle of radius 12
at z = 16, to radius 4. The round is the torus swept by the circle of radius 4 tangent to both,
whose center in the section is (8, 16 − t), t = 4 tan 22.5° ≈ 1.657: in the section, an arc of 45°
from (12, 16 − t) on the cylinder to (12 − t/√2, 16 + t/√2) on the cone. It runs the edge's whole
length and ends in the two flat faces at φ = 0° and φ = 270°, whose corners it rounds the same way.

**As a set:**

$$\{\, (\rho, z) \in P \setminus (Q \setminus D),\ 0^\circ \le \varphi \le 270^\circ \,\}$$

where P is the polygon above, Q = poly((8, 16 − t), (12, 16 − t), (12, 16), (12 − t/√2, 16 + t/√2))
the corner's kite, and D the disk of radius 4 about (8, 16 − t), all in (ρ, z).
