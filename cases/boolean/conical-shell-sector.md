+++
title = "A sector of a conical shell"
tags = ["boolean", "cut", "cone", "revolve", "analytic"]
origin = "OCCT boolean/gdml_public/B9"

[volume]
value = 452.3893421169302
exact = "(240/360)·216π = 144π, a sector of the conical shell"
tex = '\tfrac{240}{360}\cdot 216\pi = 144\pi'

[area]
value = 498.28844261918744
exact = "(2/3)(72√5π + 54π) for the side and the two rings, and two flat parallelograms of 3 across (along the radius) and 8 high, 24 each: 48√5π + 36π + 48"
tex = '\tfrac23\left(72\sqrt5\,\pi + 54\pi\right) + 2\cdot 24 = 48\sqrt5\,\pi + 36\pi + 48'
+++

Part of a turn of a conical shell. In the xz plane (the first number is x and the second z), take the parallelogram with corners (5, 0), (8, 0), (4, 8) and (1, 8), whose sides are the lines ρ = 8 − z/2 and ρ = 5 − z/2 and the two flat ends. Turn it 240° about the z axis, right-handed: from +x toward +y, through 120°, 180° and 240°. The solid has two flat ends, each a parallelogram, one in the half plane at 0° (the section itself) and one in the half plane at 240°; nothing of it lies in the remaining 120°.

**As a set:**

$$\{\,5 - \tfrac{z}{2} \le \rho \le 8 - \tfrac{z}{2},\ 0 \le z \le 8,\ 0 \le \varphi \le 240^\circ\,\}$$
