+++
title = "A prism with a Bézier side, the edge along it rounded"
tags = ["fillet", "bezier", "extrude", "convex"]
origin = "OCCT tests/bugs/modalg_7/bug23902"
settled = false
+++

A prism: the plane region in the xy plane bounded by three curves, extruded straight up from
z = 0 to z = 6. Its outline runs counterclockwise from the origin: the straight segment from
(0, 0) to (12, 0); then the cubic Bézier curve with control points (12, 0), (16, 3), (14, 9) and
(6, 12), written $B(t) = (1-t)^3 P_0 + 3t(1-t)^2 P_1 + 3t^2(1-t) P_2 + t^3 P_3$ for t from 0 to 1,
which bulges outward and has no inflection; then the straight segment from (6, 12) back to the
origin. Its side over the curve is a vertical surface ruled by the curve, and the prism's top and
bottom are flat.

Round the one edge where the top face meets that curved side, the curve $B$ raised to z = 6, to
radius 1: a quarter-round of radius 1 section, tangent to the top and to the curved side all the
way along it, the blend a ball of radius 1 sweeps as it rolls along the edge touching both. At
its ends, where the curved side meets the two flat sides at the points (12, 0) and (6, 12), the
round runs out onto their planes, which trim it square. The other edges are left sharp.

The round has no closed form, so the case is unsettled until two kernels agree.
