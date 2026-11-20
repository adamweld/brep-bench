+++
title = "Two plates, a half-disk, a quarter ring and five blind holes"
tags = ["boolean", "union", "cut", "cylinder", "planar", "analytic"]
origin = "OCCT boolean/gdml_public/A8"

[volume]
value = 195.06689916772578
exact = "100 − π + 12.5π + 3π + 54 − πΣrᵢ²dᵢ, Σrᵢ²dᵢ = 1.428: the first plate less its hole, the half-disk (½π5²), the quarter ring (¼π(4² − 2²)), the second plate 3 × 9 × 2 less the five holes"
tex = '100 - \pi + \tfrac12\pi 5^2 + \tfrac14\pi\left(4^2 - 2^2\right) + 54 - \pi\sum r_i^2 d_i'

[area]
value = 403.13981272242245
exact = "bottom 100 + 12.5π − π, top of the first plate and the half-disk 73 + 8.5π (less the hole, the ring's 3π and the second plate's 27), the first plate's three free sides 30, the half-disk's side 5π, the hole's wall 2π, the ring's top 3π, its curved sides 3π and its two flat ends 4, the second plate's top and hole floors 27, its sides 48 and the holes' walls Σ2πrᵢdᵢ"
tex = '(100 + 11.5\pi) + (73 + 8.5\pi) + 30 + 5\pi + 2\pi + (3\pi + 3\pi + 4) + 27 + 48 + 2\pi\sum r_i d_i'
+++

A stack of plates joined into one solid, standing on the plane z = 0.

The first plate is the box from (0, 0, 0) to (10, 10, 1). A half-disk of the same thickness is joined to its side x = 10: the part with x ≥ 10 of the disk of radius 5 centered at (10, 5), from z = 0 to z = 1; its straight edge is the plate's whole side. A hole of radius 1 goes through the plate along the vertical line through (5, 5).

On the plate's top, z = 1, stands a quarter ring from z = 1 to z = 2: the part with x ≥ 0 and y ≥ 0 of the annulus about the vertical axis through the origin between radii 2 and 4. Its two flat ends lie in the planes x = 0 and y = 0, which are also the plate's sides.

A second plate, the box from (6.5, 0.5, 1) to (9.5, 9.5, 3), 2 thick, stands on the first plate's top. Five round blind holes are drilled into its top face z = 3, each along a vertical line through (8, y), each with its flat floor at the depth given:

| y | radius | depth |
|---|---|---|
| 1.5 | 0.3 | 0.5 |
| 3 | 0.4 | 1 |
| 5 | 0.5 | 1.5 |
| 7 | 0.6 | 1.8 |
| 8.5 | 0.5 | 0.8 |

None of the holes reaches the second plate's bottom, and the second plate does not reach the through hole.
