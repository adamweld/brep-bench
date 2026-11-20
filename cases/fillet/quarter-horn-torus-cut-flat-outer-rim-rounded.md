+++
title = "A quarter turn of a horn torus's tube, cut flat, with its outer rim rounded"
tags = ["fillet", "torus", "planar", "singular", "analytic"]
origin = "OCCT blend/simple/E5"

[volume]
value = 253.19765640254734
exact = "(π/2)·∮(ρ²/2)dz round the section's boundary (Pappus, a quarter turn): the section is the disk of radius 4 about (4, 0) cut at z = 2 (z ≤ 2), its corner (4 + 2√3, 2) rounded by a circle of radius 1 centered at (4 + 2√2, 1), touching z = 2 at (4 + 2√2, 2) and the disk's circle at (4 + 8√2/3, 4/3)"
tex = '\frac{\pi}{2}\oint \tfrac{\rho^2}{2}\,dz'

[area]
value = 227.83953182281188
exact = "(π/2)·∫ρ ds along the section's circle, round arc and chord (Pappus), plus the two flat faces, each the section's area 40.3624487200651 = 15·arctan(√2/4)/2 + 2√3 + 3√2 + 115π/12"
tex = '\frac{\pi}{2}\int \rho\,ds + 2\left(\tfrac{15}{2}\arctan\tfrac{\sqrt2}{4} + 2\sqrt3 + 3\sqrt2 + \tfrac{115\pi}{12}\right)'
+++

A quarter turn about the z axis of a horn torus's tube cut flat: the points with x ≥ 0, y ≥ 0,
(ρ − 4)² + z² ≤ 16 and z ≤ 2, where ρ = √(x² + y²). The torus has ring radius 4 equal to its tube
radius 4, so its surface closes to a point at the origin, on the axis, where the two radial faces
x = 0 and y = 0 and the torus all meet. The solid is bounded by that torus, by the flat top z = 2, a
quarter of an annulus from ρ = 4 − 2√3 to ρ = 4 + 2√3, and by the radial faces. Round the quarter
circle of radius 4 + 2√3 where the top meets the torus, the outer one, to radius 1, and no other
edge. The round is a quarter of a torus's tube, tangent to the top and to the torus, and it ends
square on the two radial faces.
