+++
title = "A quarter of a cone frustum with its top rim rounded"
tags = ["fillet", "cone", "torus", "planar", "analytic"]
origin = "OCCT blend/simple/B8"

[volume]
value = 301.05341808565154
exact = "(π/2)·∮(ρ²/2)dz round the section's boundary (Pappus, a quarter turn): the section is the polygon (0,0), (8,0), (5,9), (0,9) with its corner (5, 9) rounded by a circle of radius 2 centered at (ρc, 7), ρc = (17 − 2√10)/3, touching z = 9 at (ρc, 9) and the cone's side at (ρc + 6/√10, 7 + 2/√10)"
tex = '\frac{\pi}{2}\oint \tfrac{\rho^2}{2}\,dz'

[area]
value = 279.72512954080499
exact = "(π/2)·∫ρ ds along the section's cone side, round arc and top (Pappus), plus the two flat faces, each the section's area 58.1150546645720 = 359/6 + π − 4√10/3 − 2·arctan(1/3)"
tex = '\frac{\pi}{2}\int \rho\,ds + 2\left(\tfrac{359}{6} + \pi - \tfrac{4\sqrt{10}}{3} - 2\arctan\tfrac13\right)'
+++

A quarter turn of a cone frustum: the points with x ≥ 0, y ≥ 0, 0 ≤ z ≤ 9 and
ρ ≤ 8 − z/3, where ρ = √(x² + y²). Its bottom (radius 8) and top (radius 5) are flat, its two
radial faces are the planes x = 0 and y = 0, and it has a cone face between them. Round the quarter
circle of radius 5 where the top meets the cone to radius 2, and no other edge. The round is a quarter
of a torus's tube, tangent to the top and to the cone, and it ends square on the two radial
faces.
