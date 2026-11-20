+++
title = "A prism on a closed cubic Bézier loop with a corner, its vertical corner edge rounded"
tags = ["fillet", "bezier", "extrude", "convex", "freeform"]
origin = "OCCT bugs/modalg_7/bug30940_2, OCCT bugs/modalg_7/bug30940_3, OCCT bugs/modalg_7/bug30940_5"
settled = false
+++

A teardrop prism. Its profile in the xy plane is bounded by one cubic Bézier curve that starts and ends at the origin, with control points (0, 0), (20, 20), (20, −20) and (0, 0), $B(t) = (60t(1-t),\ 60t(1-t)(1-2t))$, which runs once round from the origin out to (15, 0) and back, clockwise. The loop is closed with a corner at the origin, where the curve leaves along (1, 1) and arrives along (−1, 1), a right angle between the two ends. Extrude the profile straight up the z axis from z = 0 to z = 10.

Round the one vertical edge, above the corner, with a fillet of radius 1: a blend between the two ends of the Bézier wall that rolls a ball of radius 1 between them, replacing the sharp edge with a surface tangent to both and ending flat on the top and base.
