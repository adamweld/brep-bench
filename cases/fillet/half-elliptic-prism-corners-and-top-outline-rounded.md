+++
title = "A half-elliptic prism with its two upright corners rounded, then its whole top outline"
tags = ["fillet", "corner", "elliptic-cylinder", "planar", "chain"]
settled = false
origin = "OCCT tests/blend/simple/F5, OCCT tests/blend/simple/F7"
+++

The half of an elliptic cylinder: the half-ellipse x²/64 + y²/16 ≤ 1 with y ≥ 0, semi-axes 8 and 4,
its straight side the major axis from (−8, 0) to (8, 0), extruded from z = 0 to z = 4. It has a flat
bottom and top, a flat vertical side in the plane y = 0 and a curved wall, the elliptic cylinder.
The wall meets the side in two upright edges, at (±8, 0), at a right angle.

Rounds meeting at the corners where the top outline meets those upright edges can close more than
one way, so the corners are the ones these rounds make one after another:

1. Round the two upright edges to radius 3/2: each a quarter-turn round, a vertical surface
   tangent to the flat side and to the curved wall, which ends square on the top and the bottom.
   (The ellipse's tightest curve, at its ends, has radius of curvature 16/8 = 2, more than 3/2.)
   The top face's outline is then one closed chain, tangent all along: the straight side from
   one round to the other, a round's plan arc, the elliptic arc, and the other round's plan arc.
2. Round that whole chain on the top face, all the way round, to radius 1/2: a ball of radius 1/2
   rolling along the chain, touching the top and whatever is below it (the flat side, the upright
   rounds, the curved wall) from inside.

The solid has no closed form for its volume or area, so the case is unsettled until two kernels
agree.
