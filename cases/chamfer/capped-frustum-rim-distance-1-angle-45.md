+++
title = "A frustum capped by a ball, its rim chamfered by a distance of 1 and an angle of 45°"
tags = ["chamfer", "distance-angle", "sphere", "cone"]

[volume]
value = 127.5907201864641
exact = "Pappus on the half-section: V = V₀ − 2π(M_t + M_s), V₀ = 2π(M_p + M_c), M_p the ∫ρ dA of the polygon (0, 0), (2, 0), (4, 3), (0, 5), M_c of the ball's segment cut off by the chord from (4, 3) to its pole, M_t of the triangle P, E = (4, 3), Q, M_s of the ball's segment cut off by the chord E–Q; P = E − d(2, 3)/√13, Q the line from P at angle β from the cone toward the axis, where it meets the ball; d = 1, β = 45°; V₀ = 142.41886696273727"

[area]
value = 127.68324248488022
exact = "A₀ − π(ρ_P + ρ_E)d − 2π·5(z_Q − z_E) + π(ρ_P + ρ_Q)|PQ|, A₀ = π·2² + π(2 + 4)√13 + 2π·5·2 = 143.3612640809889: the cone's band from P to the rim and the ball's zone from the rim to Q taken, the chamfer's cone added"
+++

A frustum on the z axis, widening upward from the disk of radius 2 at z = 0 to radius 4 at z = 3, capped by the part of
the ball of radius 5 about the origin above z = 3: the ball meets the frustum's cone in its rim, the circle of radius 4
at z = 3, at about 70.5°, a convex edge.

Chamfer the rim by a distance and an angle: 1 down the cone from the rim, the chamfer making 45° with the cone's
side, turned toward the z axis. In the plane through the z axis the cone's side is the straight line from (2, 0) up to
the rim (4, 3). The chamfer starts on it at P, 1 from the rim, (3.445, 2.168) in (ρ, z), and is the straight line
from P that makes 45° with the cone's line continued toward the rim, turned toward the axis, until it meets the ball, 1.795
along, at Q ≈ (3.093, 3.928). The chamfer is the cone that line sweeps about the z axis, and the solid is the capped
frustum less the ring between the rim and that cone.

The angle is measured from the cone's side, which is straight in that plane, so it is the same angle everywhere along
the chamfer's start.
