+++
title = "A domed button shrunk by 1"
tags = ["offset", "shrink", "revolved", "cone", "ellipsoid", "concave-edge", "unsettled"]
origin = "OCCT tests/offset/faces_type_i/D7, OCCT tests/offset/shape_type_i/D7"
settled = false
+++

The button: the solid of revolution about the z axis of the region of the half-plane ρ ≥ 0 (ρ the
distance from the axis) bounded by the axis from z = 0 to z = 5, the base, ρ from 0 to 6 at z = 0,
the straight line from (ρ, z) = (6, 0) to (4, 2), the flare, and the quarter of the ellipse (ρ/4)² +
((z − 2)/3)² = 1 from (4, 2) to (0, 5), the dome. The flare meets the dome at a concave edge, the
circle ρ = 4 at z = 2, and the dome rises from it vertically. Shrink it by 1: the solid of every
point at least 1 from the button's outside.

Every face moves 1 inward and the convex edge round the base's rim stays sharp. The concave edge
between the flare and the dome becomes a round: a piece of a torus of radius 1 about that edge's
circle, concave, joining the moved flare to the moved dome.

**As a set:** the button less everything within 1 of its outside.
