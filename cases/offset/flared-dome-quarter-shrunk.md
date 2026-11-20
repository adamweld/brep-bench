+++
title = "A flared dome turned a quarter of a turn, shrunk by 1"
tags = ["offset", "shrink", "ellipse", "cone", "revolve", "concave"]
origin = "OCCT tests/offset/shape_type_i/C6"
settled = false
+++

The solid turned about the z axis, through angles φ from 0° to 90° (φ measured from +x toward +y),
from the region of the (ρ, z) half-plane bounded below by the line z = 0 from ρ = 0 to ρ = 10, on
the outside by the straight line from (ρ, z) = (10, 0) to (6, 4) and then by the quarter of the
ellipse (ρ/6)² + ((z − 4)/8)² = 1 from (6, 4) up to (0, 12), and on the inside by the axis. So it is
a flat base disk of radius 10, a cone flaring in to radius 6 at height 4, and a dome, a quarter of
an ellipsoid of semi-axes 6, 6 and 8, rising from there to a pole at (0, 0, 12). The cone and the
dome meet at a crease, the circle ρ = 6, z = 4, where the surface turns inward and then upward: a
concave edge. Its other faces are two flat cut faces, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in
the half-plane x = 0, y ≥ 0 (φ = 90°), which meet along the z axis, from z = 0 to z = 12, in a
convex edge.

Shrink it by 1: the solid of every point at least 1 from the solid's outside.

The base moves up to z = 1; the cone moves in along its normals, to the cone ρ + z = 10 − √2; the
dome moves in as the surface of every point 1 inside the ellipsoid; and the two cut faces move in 1,
to the planes y = 1 and x = 1, which meet along the line x = y = 1. At the crease, a concave edge,
the moved cone and dome part, and the gap is closed by the part of a torus of tube radius 1 about
the crease's circle, the surface of every point 1 from it. The offset of an ellipse has no closed
form, so the case is unsettled until two kernels agree.
