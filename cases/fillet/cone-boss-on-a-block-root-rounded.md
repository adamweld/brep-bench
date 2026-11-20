+++
title = "A cone frustum standing on a block, its root rounded"
tags = ["fillet", "concave", "cone", "torus", "analytic"]
origin = "OCCT tests/blend/bfuseblend/A2"

[volume]
value = 16988.47550223934
exact = "16000 + 312π (the block and the frustum) + 2π·M, where M = 94/5 − 20α + 16/15 (α = atan(4/3)) is the moment about the axis of the round's section (Pappus): 16000 + 5276π/15 − 40π·atan(4/3)"
tex = '16000 + 312\pi + 2\pi\left(\tfrac{94}{5} - 20\alpha + \tfrac{16}{15}\right) = 16000 + \tfrac{5276\pi}{15} - 40\pi\arctan\tfrac43'

[area]
value = 4942.916732073467
exact = "3200 (the block's bottom and sides) + (1600 − 100π) (its top outside the round) + (120π − 17.4π) (the cone less its strip, 1 along the slant from the root, of area π(9 + 8.4)·1) + 9π (the frustum's top) + the round, 20π² − 40π·atan(3/4) − 16π/5 (a torus patch, 2π∫ρ ds along the arc of radius 2): 4800 + 42π/5 + 20π² − 40π·atan(3/4)"
tex = '4800 + \tfrac{42\pi}{5} + 20\pi^2 - 40\pi\arctan\tfrac34'
+++

A block from (−20, −20, 0) to (20, 20, 10), and standing on its top face a cone frustum about the z axis: radius 9 at the bottom, at z = 10, radius 3 at the top, at z = 18, so 3 in radius for every 4 in height. The two are one solid.

Round the circle where the frustum's cone meets the block's top, the circle of radius 9 at z = 10, to radius 2. The round is concave, a ball of radius 2 rolling round the root touching the top face and the cone. It meets the top face on the circle of radius 10 (at z = 10) and the cone on the circle 1 along the slant from the root, at radius 8.4 and z = 10.8; its section through the axis is the arc of the circle of radius 2 about (ρ, z) = (10, 12), a torus patch.

**As a set:** in the plane through the axis, with $z' = z - 10$, the filled corner is the points with $0 \le z' $, $\rho \le 10$, above the cone line $\rho = 9 - \tfrac34 z'$, on the root's side of the line through $(8.4, 0.8)$ and $(10, 2)$ and outside the disk about $(10, 2)$ of radius 2, turned about the axis.
