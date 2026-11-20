+++
title = "A half-elliptic prism with its curved top edge rounded"
tags = ["fillet", "elliptic-cylinder", "planar", "analytic"]
origin = "OCCT tests/blend/simple/F4, OCCT tests/blend/simple/F6, OCCT tests/blend/simple/F8, OCCT tests/blend/simple/G1, OCCT tests/blend/simple/J6"

[volume]
value = 110.12919013401433
exact = "(πab/2)h − (A·L − M·π) by the generalized Pappus, a = 6, b = 3, h = 4, r = 1, A = r²(1 − π/4) the removed corner's section, M = r³(5/6 − π/4) its moment about the wall, L = 2a·E(1 − b²/a²) = 14.532672330821514 the arc's length (E the complete elliptic integral of the second kind), π the turn of the tangent along the arc"

[area]
value = 155.79026384866248
exact = "πab/2 (bottom) + 2ah − 2A (the flat side, less the round's end sections) + L(h − r) (curved wall) + (πr/2)L − π²r²/2 + πr² (the round) + πab/2 − rL + πr²/2 (top)"
+++

The half of an elliptic cylinder: the half-ellipse x²/36 + y²/9 ≤ 1 with y ≥ 0, semi-axes 6 and 3,
its straight side the major axis from (−6, 0) to (6, 0), extruded from z = 0 to z = 4. It has a flat
bottom and top, a flat vertical side in the plane y = 0, and a curved wall, the elliptic cylinder,
which meets the flat side at the ends (±6, 0) at a right angle.

Round the one edge where the curved wall meets the top, the half-ellipse arc at z = 4, to radius 1.
The round is the surface a ball of radius 1 sweeps as it rolls along that edge, touching the top
and the curved wall from inside. At the arc's ends, (±6, 0, 4), the wall is square to the flat
side, so the round runs out square onto it, a quarter circle of radius 1 in the plane y = 0, and
the straight edge along the top of the flat side stays sharp. (The ellipse curves tightest at its
ends, with radius of curvature 9/6 = 1.5, more than 1, so the ball always fits.)

**As a set:**

$$\{\,(x, y) \in D,\ 0 \le z \le 4\,\} \setminus \{\,(x, y) \in D,\ z \ge 3,\ d \le 1,\ (1 - d)^2 + (z - 3)^2 \ge 1\,\}$$

where D is the half-ellipse and d the distance from (x, y) to the ellipse x²/36 + y²/9 = 1.
