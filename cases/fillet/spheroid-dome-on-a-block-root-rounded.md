+++
title = "A spheroid dome standing on a block, its root rounded"
tags = ["fillet", "concave", "spheroid", "revolved"]
origin = "OCCT tests/blend/bfuseblend/A8, OCCT tests/blend/bfuseblend/A9"
settled = false
+++

A block from (−30, −30, 0) to (30, 30, 10), and standing on its top face a dome: the half, above z = 10, of the spheroid about the z axis with centre (0, 0, 10), radius 8 across (ρ² / 8² + (z − 10)² / 12² ≤ 1) and 12 up. The dome's profile is a half ellipse, vertical where it meets the top face, on the circle of radius 8. The two are one solid.

Round that circle to radius 2. The round is concave, a ball of radius 2 rolling round the root touching the top face and the dome, its centre always at height 2 above the block (z = 12) and 2 from the dome's surface, so on the circle round the axis that the offset of the half ellipse reaches at z = 12. It is a revolved surface, not a torus: its section through the axis is a circular arc of radius 2 from the top face to the spheroid, whose tangent point on the spheroid is where the spheroid's normal passes through the ball's centre.

The round's volume and area have no closed form that does not need the root of an equation (the ball's centre at height 2 and distance 2 from an ellipse), so the case is unsettled until two kernels agree.

**As a set:** with $z' = z - 10$, the union of the block and the dome $\{\rho^2/64 + z'^2/144 \le 1,\ z' \ge 0\}$, closed by a ball of radius 2 (grown by 2, then shrunk by 2): the root is the solid's only concave place, so the closing adds the round and nothing else.
