+++
title = "A block pierced by a thin conical ring that rises above its top"
tags = ["boolean", "union", "analytic", "revolve", "cone"]
origin = "OCCT boolean/bfuse_simple/K1"

[volume]
value = 2105.9165523210273
exact = "20·20·5 + 2π·(118/7)"
tex = '20\cdot 20\cdot 5 + 2\pi\cdot \tfrac{118}{7}'

[area]
value = 1417.5741412786688
exact = "2·(400 + 100 + 100) − 55π/7 + 9π + (2360√2/49)π"
tex = '2\,(400 + 100 + 100) - \tfrac{55\pi}{7} + 9\pi + \tfrac{2360\sqrt2}{49}\pi'
+++

A block from (−10, −10, 0) to (10, 10, 5), its faces planes. And a thin ring turned about the z axis: the parallelogram with corners (ρ, z) = (3, 2), (4, 2), (5, 9) and (4, 9) in the half plane through the axis, turned all the way round. It is a slanting tube, 1 thick measured across, whose inner wall is the cone ρ = 3 + (z − 2)/7 and whose outer wall is the cone ρ = 4 + (z − 2)/7, from z = 2 to z = 9. Unite them.

The ring's axis passes through the block's interior and its lower part, from z = 2 to z = 5, lies wholly inside the block, clear of its sides. The upper part, from z = 5 to z = 9, rises above the top face; there the ring's two walls cross the top's plane in two circles of radii 24/7 and 31/7, and the union is the block with the slanting tube standing on its top.

**As a set:**

$$[-10,10]^2\times[0,5] \cup \{\,3 + \tfrac{z-2}{7} \le \rho \le 4 + \tfrac{z-2}{7},\ 2 \le z \le 9\,\}$$
