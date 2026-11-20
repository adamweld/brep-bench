+++
title = "A hemisphere standing on a block, its root rounded"
tags = ["fillet", "concave", "sphere", "torus", "analytic"]
origin = "OCCT tests/blend/bfuseblend/A3"

[volume]
value = 37220.60855810321
exact = "36000 + (2/3)π·8³ (the block and the hemisphere) + 2π·M, M = 200 − 150·atan(12/5) the moment about the axis of the round's section (Pappus; the triangle of the ball's centre, the circle's centre and its tangent point on the block, less the ball's sector and the circle's): 36000 + 2224π/3 − 300π·atan(12/5)"
tex = '36000 + \tfrac{1024\pi}{3} + 2\pi\left(200 - 150\arctan\tfrac{12}{5}\right) = 36000 + \tfrac{2224\pi}{3} - 300\pi\arctan\tfrac{12}{5}'

[area]
value = 9741.750623558590
exact = "6000 (the block's bottom and sides) + (3600 − 144π) (its top outside the round) + (128π − 640π/13) (the hemisphere less the zone 40/13 high under the round) + the round, 60π² − 120π·atan(5/12) − 400π/13 (a torus patch, 2π∫ρ ds along the arc of radius 5): 9600 − 96π − 120π·atan(5/12) + 60π²"
tex = '9600 - 96\pi - 120\pi\arctan\tfrac{5}{12} + 60\pi^2'
+++

A block from (−30, −30, 0) to (30, 30, 10), and standing on its top face a hemisphere of radius 8: the part of the ball of radius 8 about (0, 0, 10) above z = 10. The two are one solid; the ball's side is vertical where it meets the top face.

Round the circle of radius 8 where the hemisphere meets the block's top, to radius 5. The round is concave, a ball of radius 5 rolling round the root touching the top face and the hemisphere. It meets the top face on the circle of radius 12 at z = 10 and the hemisphere on the circle at height 40/13 above the top face, radius 96/13 (on the line from the hemisphere's centre through the rolling ball's centre, which is at radius 12 and height 5); its section through the axis is a torus patch.

**As a set:** in the plane through the axis, with $z' = z - 10$, the filled corner is the points with $z' \ge 0$, $\rho \le 12$, $z' \le 5\rho/12$, outside the hemisphere's ball $\rho^2 + z'^2 > 8^2$ and outside the disk about $(12, 5)$ of radius 5, turned about the axis.
