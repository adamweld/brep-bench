+++
title = "A narrow cylinder cut off by a larger inclined cylinder, the cut's rim rounded"
tags = ["fillet", "cylinder", "saddle", "convex", "inclined"]
settled = false
origin = "OCCT tests/blend/simple/K4"
+++

A cylinder of radius 2 about the z axis, from z = 0 to z = 15, less the solid cylinder of radius 5
about the line through (0, 0, 13) in the direction (1, 0, 1)/√2 (the tool, a longer, wider cylinder
leaning at 45° across the top of the first, taken to run far beyond it both ways). The tool
removes the top of the narrow cylinder, leaving a stub standing on z = 0 whose top is a scoop of
the tool's wall: on the plane y = 0 the scoop runs from (−2, 0, 3.93) to (2, 0, 7.93), and the
stub's side meets it in a closed curve all the way round.

Round that curve, where the stub's side meets the scoop, to radius 1/2: a ball of that radius
rolling round it, touching the side and the scoop from inside. The bottom edge stays sharp.

The section of the round changes all the way round the curve, and it has no closed form, so the
case is unsettled until two kernels agree.
