+++
title = "Three quarters of a spheroid dome, its base rim rounded with a radius growing along it"
tags = ["fillet", "variable-radius", "ellipsoid", "planar", "revolve", "unsettled"]
origin = "OCCT tests/blend/buildevol/D5, OCCT tests/blend/buildevol/D6"
settled = false
+++

A dome sector: the quarter of the ellipse (ρ/10)² + (z/5)² ≤ 1 with ρ ≥ 0 and z ≥ 0, turned about the z
axis from φ = 0° to φ = 270° (φ measured from +x toward +y). It is three quarters of a half spheroid,
5 high on a flat base of radius 10 in the plane z = 0, where the spheroid stands square to the base.
It has a flat base, three quarters of a disc; a domed face, three quarters of the spheroid; and two
flat end faces, in the half-planes φ = 0° and φ = 270°, each a quarter of an ellipse (the dome's
section through its axis).

Round the one edge where the base meets the spheroid, the three-quarter circle of radius 10 at
z = 0, with a radius that grows along it: r = 1 + 2φ/(3π), so 1 where the edge meets the end face at
φ = 0° and 2 where it meets the end face at φ = 270°, growing evenly with the angle. The other edges
stay sharp.

The round's sections square to the edge are circular: in the plane through the z axis at each angle
φ, the round's section is an arc of the circle of radius r(φ) that lies inside the dome and touches
the base and the ellipse: its center at height r, at the distance r inside the ellipse. (The
ellipse's radius of curvature at the base is 5²/10 = 2.5, more than the largest radius.) The round
runs the whole length of the edge and ends in the two end faces, each of which it meets in such an
arc, of radius 1 at φ = 0° and 2 at φ = 270°.

The offset of an ellipse has no closed form, so the volume and area have none, and the case is
unsettled until two kernels agree.
