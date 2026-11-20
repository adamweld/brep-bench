+++
title = "A cylinder with a cross hole near its end, the hole's rim on one side rounded"
tags = ["fillet", "cylinder", "saddle", "convex"]
settled = false
origin = "OCCT tests/blend/simple/K2, OCCT tests/blend/simple/K3"
+++

A cylinder of radius 5 about the z axis, from z = 0 to z = 20, with a hole of radius 2 drilled
clean through it across a diameter: the hole's axis is the line y = 0, z = 4 along the x axis, so
the hole runs from one side of the cylinder to the other and stays clear of the bottom (it spans
z = 2 to z = 6).

The hole's wall meets the cylinder's side in two closed saddle-shaped curves, one where the hole
leaves the cylinder at x > 0 and one at x < 0. Round the one at x > 0 to radius 1/2: a ball of
that radius rolling round the curve, touching the cylinder's side and the hole's wall from
inside. Everything else, the other curve and both rims, stays sharp.

The round's section changes all the way round, and it has no closed form, so the case is
unsettled until two kernels agree.
