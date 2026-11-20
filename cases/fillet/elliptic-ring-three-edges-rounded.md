+++
title = "A ring with a quarter-ellipse section turned three quarters of a turn, its three circular edges rounded"
tags = ["fillet", "revolve", "cylinder", "planar", "surface-of-revolution"]
settled = false
origin = "OCCT tests/blend/simple/H2, OCCT tests/blend/simple/H3, OCCT tests/blend/simple/H4, OCCT tests/blend/simple/H5, OCCT tests/blend/simple/J7"
+++

In the half-plane y = 0, x ≥ 0 (ρ = x), the region P bounded by the line from (ρ, z) = (3, 0) to
(6, 0), the quarter of the ellipse ((ρ − 3)/3)² + (z/2)² = 1 from (6, 0) to (3, 2) (centered at
(3, 0), semi-axes 3 along ρ and 2 along z), and the line from (3, 2) down to (3, 0). P is turned
about the z axis from φ = 0° to φ = 270°, right-handed. The solid has a flat underside, an annulus
at z = 0 between ρ = 3 and ρ = 6; an inner wall, the cylinder of radius 3 from z = 0 to z = 2; a
crown, the surface the ellipse arc turns out, which is no quadric; and two flat ends, in the
half-planes φ = 0° and φ = 270°.

Round all three circular edges, each three quarters of a circle, to radius 1/2: where the underside
meets the crown (radius 6), where the crown meets the inner wall (radius 3, at z = 2) and where
the inner wall meets the underside (radius 3, at z = 0). All three are convex, at right angles in
the section. Each round is the blend a ball of radius 1/2 sweeps as it rolls along its edge,
touching both faces from inside; it runs the whole length of its edge and ends in the flat end
faces. The three rounds do not meet one another.

The rounds against the crown have no closed form, so the case is unsettled until two kernels
agree.
