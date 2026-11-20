+++
title = "A cylinder cut aslant with its elliptical rim rounded with a radius that varies round it"
tags = ["fillet", "variable-radius", "cylinder", "planar", "closed-edge", "ellipse", "unsettled"]
origin = "OCCT tests/blend/buildevol/F4, OCCT tests/blend/buildevol/F1"
settled = false
+++

A cylinder of radius 6 on the z axis from z = 0, its top cut by the plane z = 12 + 0.4x: it is 9.6
high at x = −6 and 14.4 high at x = 6, its top face an ellipse with the long axis 12.9 and the short
12. Round the elliptical edge around that top face with a radius that varies all the way round it: at
the angle φ about the z axis, measured from +x toward +y, r = 1.5 + 0.5 cos φ. So the radius is 2 on
the high side, 1 on the low side, and varies smoothly between.

The round's sections square to the edge are circular: at each point of the edge, in the plane through
it square to the edge, the round's section is an arc of the circle of radius r(φ) that lies inside the
solid and touches both faces' traces in that plane, the side's and the top's. The edge's angle between
the side and the top varies round it, from 68.2° on the high side to 111.8° on the low side.
