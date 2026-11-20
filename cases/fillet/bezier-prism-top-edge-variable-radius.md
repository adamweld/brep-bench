+++
title = "A prism on a Bézier-bounded profile with its curved top edge rounded with a growing radius"
tags = ["fillet", "variable-radius", "bezier", "extrusion", "freeform", "unsettled"]
origin = "OCCT tests/blend/buildevol/D1, OCCT tests/blend/buildevol/D2, OCCT tests/blend/buildevol/D3, OCCT tests/blend/buildevol/D4"
settled = false
+++

A prism whose profile in the xy plane is bounded by the straight segment from (0, 0) to (20, 0) and
by the cubic Bézier curve B(t) with control points (0, 0), (4, 14), (16, 14), (20, 0), t from 0 to 1,
which leaves (0, 0) upward and comes back down to (20, 0): a dome 10.5 high at t = 1/2. Extrude it
from z = 0 to z = 10. It has a flat bottom and top, a flat wall on y = 0 and a curved wall standing
on the Bézier curve.

Round the curved edge where the top meets the curved wall, from (0, 0, 10) to (20, 0, 10), with a
radius that grows along it with the curve's parameter: r = 1 + 2t, so 1 where it starts at (0, 0, 10)
and 3 where it ends at (20, 0, 10). The round ends on the flat wall.

The round's sections square to the edge are circular: at each point B(t) of the edge, in the plane
through it square to the edge (the plane that holds the z direction and the curve's normal there),
the round's section is the quarter circle of radius r(t), tangent to the top and to the curved wall,
its center at the point r(t) below the top and r(t) inside the wall from B(t) along the curve's
inward normal. (The curve's radius of curvature is 6.9 at its middle and larger elsewhere, more than
the round's radius, which is at most 3.)
