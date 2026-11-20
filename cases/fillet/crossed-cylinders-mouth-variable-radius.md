+++
title = "The mouth of a cross hole through a cylinder rounded with a radius that varies round it"
tags = ["fillet", "variable-radius", "cylinder", "cut", "closed-edge", "unsettled"]
origin = "OCCT tests/blend/buildevol/F7, OCCT tests/blend/buildevol/F8, OCCT tests/blend/buildevol/G1"
settled = false
+++

A cylinder of radius 8 on the z axis from z = −12 to z = 12, with a hole of radius 3 drilled right
through it along the x axis: the cylinder of radius 3 about the x axis, from x = −10 to x = 10, taken
away. The hole opens in the big cylinder's side at two mouths, one on the +x side and one on the −x
side.

Round the edge around the +x mouth, where the hole's wall meets the big cylinder's side, with a radius
that varies all the way round it: at the angle α about the x axis, measured from +y toward +z, the
edge's point is (√(64 − 9 cos²α), 3 cos α, 3 sin α) and r = 0.8 + 0.4 cos α. So the radius is 1.2 at
α = 0 (the point (√55, 3, 0)), 0.4 opposite at α = 180°, and 0.8 at the top and bottom of the mouth.
Leave the −x mouth sharp.

The round's sections square to the edge are circular: at each point of the edge, in the plane
through it square to the edge, the round's section is an arc of the circle of radius r(α) that lies
inside the solid and touches the hole's wall and the big cylinder's side in that plane. The edge's
angle inside the solid between the two walls is 90° where it crosses the plane y = 0 and smaller
elsewhere, down to 68° at α = 0 and at 180°.
