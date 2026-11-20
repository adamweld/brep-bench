+++
title = "A post on a block under a parabolic dome, its side drafted"
tags = ["draft", "cone", "bezier", "revolve", "boolean", "analytic"]
origin = "OCCT tests/draft/angle/G4"

[volume]
value = 10155.880146224543
exact = "2800π + π[(ρ₀³ − r₁³)/(3T) − (10 − z₀)²/(2k)], k = 0.015, T = tan 4°, z₀ = (−B + √(B² − 4AC))/(2A), A = kT², B = 1 − 12kT, C = 36k − 10 (the root of z = 10 − k(6 − zT)² below 10), ρ₀ = 6 − z₀T, r₁ = 6 − 30T: the block, ∫₀²⁰ 2πρ(10 − kρ²)dρ = 2800π, and the part of the cone above the dome, π∫ (6 − zT)² dz from z₀ to 30 less the part under the dome from z₀ to 10, π∫ (10 − z)/k dz"
tex = '2800\pi + \pi\left[\tfrac{\rho_0^3 - r_1^3}{3T} - \tfrac{(10 - z_0)^2}{2k}\right],\quad k = 0.015,\ T = \tan 4^\circ,\ \rho_0 = 6 - z_0T,\ r_1 = 6 - 30T,\ z_0 = 9.5737\ldots'

[area]
value = 3674.940914558306
exact = "560π + (π/(6k²))[(1 + 1600k²)^{3/2} − (1 + 4k²ρ₀²)^{3/2}] + π(ρ₀ + r₁)(30 − z₀)/cos 4° + πr₁²: the floor and the rim's wall, 400π + 2π·20·4, the dome's surface from ρ₀ to the rim, ∫ 2πρ√(1 + 4k²ρ²)dρ, the cone's side and the top"
tex = '560\pi + \tfrac{\pi}{6k^2}\left[(1 + 1600k^2)^{3/2} - (1 + 4k^2\rho_0^2)^{3/2}\right] + \frac{\pi(\rho_0 + r_1)(30 - z_0)}{\cos 4^\circ} + \pi r_1^2'
+++

The block: the solid turned about the z axis from the region under the quadratic Bézier arc from
(ρ, z) = (0, 10) through (10, 10) to (20, 4), above z = 0 and out to ρ = 20. The arc is the
parabola z = 10 − 0.015ρ², so the block is a flat-floored disk of radius 20 under a dome, 10 high at
the axis and 4 high at its rim.

A post stands on its axis: the cylinder of radius 6 about the z axis from z = 0 to z = 30, united
with the block. Its side shows only above the dome, where the dome is under it: from z = 10 − 0.015·36 = 9.46.

Draft the post's side by 4°, with the pull direction +z and the neutral plane z = 0, the block's
floor: the side becomes the cone through the circle of radius 6 at z = 0, leaning 4° from the z
direction, inward as it rises, its radius 6 − z·tan 4°. It still runs down into the dome, now
meeting it where z = 10 − 0.015(6 − z·tan 4°)², at z₀ ≈ 9.574, radius ρ₀ ≈ 5.331, and the top
shrinks to meet it, to radius 6 − 30·tan 4° ≈ 3.902. The block and the post's top are not drafted.

So the result is the block with a frustum of the cone standing on its dome.

**As a set:**

$$\{\, 0 \le z \le 10 - 0.015\rho^2,\ \rho \le 20 \,\} \cup \{\, \rho \le 6 - z\tan 4^\circ,\ 0 \le z \le 30 \,\}$$
