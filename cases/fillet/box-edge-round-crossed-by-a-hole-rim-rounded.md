+++
title = "A cube with one edge rounded, a hole bored into the round, the hole's rim rounded"
tags = ["fillet", "cylinder", "saddle", "convex", "round"]
settled = false
origin = "OCCT tests/blend/simple/Q4"
+++

A cube of side 10 between the origin and (10, 10, 10), its upright edge where the faces x = 10 and
y = 10 meet rounded to radius 4: a quarter cylinder about the line x = 6, y = 6, tangent to both
faces. Bore into it a hole of radius 3/2 along the horizontal diagonal direction (1, 1, 0)/√2,
the solid cylinder of that radius from its flat end, the disc centered (6, 6, 5) (on the round's
axis, inside the block), out along its axis past the surface of the round. The hole's wall
meets the round in one closed saddle-shaped curve, entirely on the round (the hole spans z = 3.5
to 6.5).

Round that curve to radius 1/2: a ball of that radius rolling round it, touching the round and the
hole's wall from inside. Everything else stays as it is.

The round's section changes all the way round, and it has no closed form, so the case is
unsettled until two kernels agree.
