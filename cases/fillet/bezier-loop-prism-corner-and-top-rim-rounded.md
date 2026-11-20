+++
title = "A prism on a closed cubic Bézier loop with a corner, its vertical corner edge rounded and then its top rim"
tags = ["fillet", "bezier", "extrude", "convex", "freeform", "corner"]
origin = "OCCT bugs/modalg_7/bug30940_4, OCCT bugs/modalg_7/bug30940_6"
settled = false
+++

The teardrop prism of `fillet/bezier-loop-prism-vertical-corner-rounded`: its profile in the xy plane bounded by the cubic Bézier curve with control points (0, 0), (20, 20), (20, −20) and (0, 0), that is $B(t) = (60t(1-t),\ 60t(1-t)(1-2t))$, extruded straight up the z axis from z = 0 to z = 10, with a right-angled corner at the origin.

Round it in two steps, one after the other. First round the vertical edge above the corner with a fillet of radius 1, as in that case. Then round the whole top rim of the result, the one smooth closed edge round the top face (the Bézier wall's top edge, the first fillet's arc and the Bézier wall again), with a fillet of radius 1 going all the way round it, made on the solid the first fillet left.
