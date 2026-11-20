+++
title = "A quarter of a spherical zone with its top rim rounded"
tags = ["fillet", "sphere", "torus", "planar", "analytic"]
origin = "OCCT blend/simple/D2"

[volume]
value = 303.71899326554657
exact = "(π/2)·∮(ρ²/2)dz round the section's boundary (Pappus, a quarter turn): the section is the region ρ ≥ 0, 2 ≤ z ≤ 7, ρ² + z² ≤ 100 with its corner (√51, 7) rounded by a circle of radius 2 centered at (√39, 5), touching z = 7 at (√39, 7) and the circle of radius 10 at (5√39/4, 25/4). Closed form π(1093 + 6π√39 − 12√39·arctan(5/√39))/12"
tex = '\frac{\pi}{12}\left(1093 + 6\pi\sqrt{39} - 12\sqrt{39}\arctan\tfrac{5}{\sqrt{39}}\right)'

[area]
value = 280.28513727212891
exact = "(π/2)·∫ρ ds along the section's sphere arc, round arc, top and bottom (Pappus), plus the two flat faces, each the section's area 43.7845422167109 = π + 9√39/2 − 4√6 − 50·arctan(√6/12) + 48·arctan(5/√39)"
tex = '\frac{\pi}{2}\int \rho\,ds + 2\left(\pi + \tfrac{9\sqrt{39}}{2} - 4\sqrt{6} - 50\arctan\tfrac{\sqrt6}{12} + 48\arctan\tfrac{5}{\sqrt{39}}\right)'
+++

A quarter turn of a band of a ball: the points with x ≥ 0, y ≥ 0, 2 ≤ z ≤ 7 and
x² + y² + z² ≤ 100. Its bottom (z = 2) and top (z = 7) are flat, its two radial faces are the planes
x = 0 and y = 0 (meeting along the axis from (0, 0, 2) to (0, 0, 7)), and the sphere of radius 10 about the
origin bounds it between them. Round the quarter circle, of radius √51, where the top meets the
sphere to radius 2, and no other edge. The round is a quarter of a torus's tube, tangent to the top
and to the sphere, and it ends square on the two radial faces.
