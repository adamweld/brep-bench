+++
title = "A flared dome turned three quarters of a turn, shrunk by 1"
tags = ["offset", "shrink", "ellipse", "cone", "revolve", "concave"]
origin = "OCCT tests/offset/faces_type_i/C8, OCCT tests/offset/shape_type_i/C8"
settled = false
+++

The solid turned about the z axis, through angles φ from 0° to 270° (φ measured from +x toward +y),
from the region of the (ρ, z) half-plane bounded below by the line z = 0 from ρ = 0 to ρ = 10, on the
outside by the straight line from (ρ, z) = (10, 0) to (6, 4) and then by the quarter of the ellipse
(ρ/6)² + ((z − 4)/8)² = 1 from (6, 4) up to (0, 12), and on the inside by the axis. So it is a flat
base disk of radius 10, a cone flaring in to radius 6 at height 4, and a dome, a quarter of an
ellipsoid of semi-axes 6, 6 and 8, rising from there to a pole at (0, 0, 12), all three quarters of a
turn. The cone and the dome meet at a crease, the circle ρ = 6, z = 4. Its other faces are two flat cut
faces, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the half-plane x = 0, y ≤ 0 (φ = 270°), which
meet along the z axis, from z = 0 to z = 12, in a concave edge.

Shrink it by 1: the solid of every point at least 1 from its outside. The base moves up to z = 1, the
cone moves in along its normals, the dome moves in as the surface of every point 1 inside the
ellipsoid, and the two cut faces move in 1, to the planes y = 1 (for x ≥ 0) and x = −1 (for y ≤ 0).
Where they would meet, round the edge of the missing quarter, the result is rounded by the quarter of
a cylinder of radius 1 about the z axis, the surface of every point 1 from the axis. The offset of an
ellipse has no closed form, so the case is unsettled until two kernels agree.

**As a set:**

$$\{\, (\rho, z) \in P,\ 0^\circ \le \varphi \le 270^\circ \,\} \ominus B_1$$

where P is the region of the half-plane described above: 0 ≤ z ≤ 4 and ρ ≤ 10 − z, or 4 ≤ z ≤ 12 and (ρ/6)² + ((z − 4)/8)² ≤ 1.
