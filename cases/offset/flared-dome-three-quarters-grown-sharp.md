+++
title = "A flared dome turned three quarters of a turn, grown by 1, its edges kept sharp"
tags = ["offset", "grow", "sharp-edge", "ellipse", "cone", "revolve", "concave"]
origin = "OCCT tests/offset/shape_type_i/C7"
settled = false
+++

The solid turned about the z axis, through angles φ from 0° to 270° (φ measured from +x toward +y),
from the region of the (ρ, z) half-plane bounded below by the line z = 0 from ρ = 0 to ρ = 10, on
the outside by the straight line from (ρ, z) = (10, 0) to (6, 4) and then by the quarter of the
ellipse (ρ/6)² + ((z − 4)/8)² = 1 from (6, 4) up to (0, 12), and on the inside by the axis. So it is
a flat base disk of radius 10, a cone flaring in to radius 6 at height 4, and a dome, a quarter of
an ellipsoid of semi-axes 6, 6 and 8, rising from there to a pole at (0, 0, 12). The cone and the
dome meet at a crease, the circle ρ = 6, z = 4, where the surface turns inward and then upward: a
concave edge. Its other faces are two flat cut faces, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in
the half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the z axis, from z = 0 to z = 12, in a
concave edge.

Grow it by 1 with its edges kept sharp: move each face 1 outward along its normal, and extend the
moved faces until they meet, so that no round is added and every edge stays where its two faces
meet.

The base moves down to z = −1; the cone moves out along its normals, to the cone ρ + z = 10 + √2,
and runs on down to the base's new plane; the dome moves out as the surface of every point 1 outside
the ellipsoid; and at the crease, a concave edge, the moved cone and dome cross and are trimmed. The
base's rim, where the cone meets the base, stays a sharp edge, the circle ρ = 11 + √2 at z = −1. The
two cut faces move out 1, the first to the plane y = −1 (for x ≥ 1) and the second to the plane x =
1 (for y ≤ −1), and are extended until they meet each other along the line x = 1, y = −1, so the
missing quarter shrinks to the part with x ≥ 1 and y ≤ −1. The offset of an ellipse has no closed
form, so the case is unsettled until two kernels agree.
