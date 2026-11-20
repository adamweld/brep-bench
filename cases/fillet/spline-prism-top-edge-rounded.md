+++
title = "A prism bounded by a cubic B-spline and its chord, the curved top edge rounded"
tags = ["fillet", "nurbs", "spline", "planar"]
settled = false
origin = "OCCT tests/blend/simple/G3, OCCT tests/blend/simple/G5, OCCT tests/blend/simple/G7, OCCT tests/blend/simple/G9"
+++

In the plane z = 0, the clamped cubic B-spline curve C with the five control points
(0, 0), (2, 6), (6, 8), (10, 5), (12, 0), the knot vector 0, 0, 0, 0, ½, 1, 1, 1, 1 and all weights
1, from (0, 0) to (12, 0), is closed by the straight chord from (12, 0) back to (0, 0). The region
they bound, with the curve above the chord, is extruded to z = 5: a prism with a flat bottom and
top, a flat upright side on the chord (the plane y = 0), and a curved wall of the B-spline cylinder
standing on C.

Round the one edge where the curved wall meets the top, the curve C at z = 5, to radius 1. The
round is the blend a ball of radius 1 sweeps as it rolls along the edge, touching the top and the
curved wall from inside. The curve meets the chord at slanting angles, so the round's ends are cut
by the plane y = 0 obliquely; the flat side's own top edge stays sharp.

The round has no closed form, so the case is unsettled until two kernels agree.
