+++
title = "A quarter ring's end disc rounded into its torus with a radius that varies round it"
tags = ["fillet", "variable-radius", "torus", "planar", "closed-edge", "unsettled"]
origin = "OCCT tests/blend/buildevol/E7, OCCT tests/blend/buildevol/E8, OCCT tests/blend/buildevol/E9"
settled = false
+++

A quarter of a ring: the solid torus whose core circle has radius 10 about the z axis, in the plane
z = 0, and whose tube has radius 3, cut to x ≥ 0 and y ≥ 0. It has the toroidal side and two flat
discs of radius 3 for ends, one in the plane y = 0 centered at (10, 0, 0) and one in the plane x = 0
centered at (0, 10, 0).

Round the circular edge around the end disc in y = 0, where the disc meets the torus, with a radius
that varies all the way round it: at the angle α about the disc's center, measured in the plane
y = 0 from +x toward +z, r = 0.75 + 0.25 cos α. So the radius is 1 at α = 0, on the outer side of
the ring, and 0.5 on the inner side.

The round's sections square to the edge are circular: at each point of the edge, in the plane through
it square to the edge (the plane that holds the y direction and the disc's radius there), the round's
section is an arc of the circle of radius r(α) that lies inside the solid and touches the disc's
trace in that plane, a straight line, and the torus's trace in that plane. The other end disc and
the other edges are left sharp.
