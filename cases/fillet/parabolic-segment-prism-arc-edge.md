+++
title = "A parabolic-segment prism with its curved top edge rounded"
tags = ["fillet", "parabolic-cylinder", "planar"]
settled = false
origin = "OCCT tests/blend/simple/F8, OCCT tests/blend/simple/G1, OCCT tests/blend/simple/F9, OCCT tests/blend/simple/G2"
+++

The parabolic segment between the parabola y = 9 − x²/4 and its chord, the segment of the x axis
from (−6, 0) to (6, 0): the region 0 ≤ y ≤ 9 − x²/4, extruded from z = 0 to z = 4. It has a flat
bottom and top, a flat upright side in the plane y = 0, and a curved wall, the parabolic cylinder,
which meets the flat side at (±6, 0) at a slant (the parabola's slope there is ∓3). The parabola
curves tightest at its vertex (0, 9), with radius of curvature 2.

Round the one edge where the curved wall meets the top, the arc of the parabola at z = 4, to
radius 1: the blend a ball of radius 1 sweeps rolling along it, touching the top and the curved
wall from inside. The round's ends are cut by the plane y = 0 obliquely; the flat side's own top
edge stays sharp.

The parabola's offset has no closed-form volume here, so the case is unsettled until two kernels
agree.
