+++
title = "A quarter turn of a torus's tube, cut flat, with its outer rim rounded"
tags = ["fillet", "torus", "planar", "analytic"]
origin = "OCCT blend/simple/E9"

[volume]
value = 95.050893240244115
exact = "(π/2)·∮(ρ²/2)dz round the section's boundary (Pappus, a quarter turn): the section is the disk of radius 2 about (6, 0) cut at z = 1 (z ≤ 1), its corner (6 + √3, 1) rounded by a circle of radius 1/2 centered at (6 + √2, 1/2), touching z = 1 at (6 + √2, 1) and the disk's circle at (6 + 4√2/3, 2/3)"
tex = '\frac{\pi}{2}\oint \tfrac{\rho^2}{2}\,dz'

[area]
value = 130.92428850419922
exact = "(π/2)·∫ρ ds along the section's circle, round arc and chord (Pappus), plus the two flat faces, each the section's area 10.0906121800163 = 15·arctan(√2/4)/8 + √3/2 + 3√2/4 + 115π/48"
tex = '\frac{\pi}{2}\int \rho\,ds + 2\left(\tfrac{15}{8}\arctan\tfrac{\sqrt2}{4} + \tfrac{\sqrt3}{2} + \tfrac{3\sqrt2}{4} + \tfrac{115\pi}{48}\right)'
+++

A quarter turn about the z axis of a torus's tube cut flat: the points with x ≥ 0, y ≥ 0,
(ρ − 6)² + z² ≤ 4 and z ≤ 1, where ρ = √(x² + y²). It is bounded by the torus (ring radius 6, tube
radius 2), by the flat top z = 1, a quarter of an annulus from ρ = 6 − √3 to ρ = 6 + √3, and by the
two radial faces x = 0 and y = 0. Round the quarter circle of radius 6 + √3 where the top meets the torus,
the outer one, to radius 1/2, and no other edge. The round is a quarter of a torus's tube, tangent to
the top and to the torus, and it ends square on the two radial faces.
