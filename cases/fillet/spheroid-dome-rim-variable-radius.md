+++
title = "A spheroid dome's base rim rounded with a radius that varies all the way round it"
tags = ["fillet", "variable-radius", "ellipsoid", "planar", "closed-edge", "revolve", "unsettled"]
origin = "OCCT tests/blend/buildevol/F2, OCCT tests/blend/buildevol/F3, OCCT tests/blend/buildevol/D5, OCCT tests/blend/buildevol/D6"
settled = false
+++

A dome: the quarter of the ellipse (ρ/10)² + (z/5)² ≤ 1 with ρ ≥ 0 and z ≥ 0, turned once about the z
axis. It is a half spheroid, 5 high on a flat circular base of radius 10 in the plane z = 0, where
the spheroid stands square to the base.

Round the circular edge around the base, where the base meets the spheroid, with a radius that varies
all the way round it: at the angle φ about the z axis, measured from +x toward +y, r = 1.5 + 0.5 cos φ.
So the radius is 2 at φ = 0, where the edge's seam is, 1 opposite it, and varies smoothly between.

The round's sections square to the edge are circular: in the plane through the z axis at each angle φ,
the round's section is an arc of the circle of radius r(φ) that lies inside the dome and touches the
base and the ellipse: its center at height r, at the distance r inside the ellipse. (The ellipse's
radius of curvature at the base is 2.5, more than the largest radius.)
