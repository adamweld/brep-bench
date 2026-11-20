+++
title = "A part-turned boss on a plate, its outer wall drafted"
tags = ["draft", "cone", "revolve", "boolean", "analytic"]
origin = "OCCT draft/angle/D6"

[volume]
value = 2599.958879300029
exact = "2304 + (2π/3)(168 − 320t + 512t²/3), t = tan 5°: the plate 24·24·4 plus the boss, (240°/360°)·π·∫₀⁸ ((5 − tz)² − 2²) dz"
tex = '2304 + \tfrac{2\pi}{3}\left(168 - 320t + \tfrac{512}{3}t^2\right),\quad t = \tan 5^\circ'

[area]
value = 1788.2082297406657
exact = "the plate's surface 2·24² + 4·24·4 less the boss's footprint (2π/3)(5² − 2²), plus the cone (2π/3)(5 + r₁)·8/cos 5° (r₁ = 5 − 8t), the inner cylinder (4π/3)·2·8, the top (2π/3)(r₁² − 2²) and two end faces 24 − 32t each"
tex = '1536 - \tfrac{2\pi}{3}(25 - 4) + \tfrac{2\pi}{3}(5 + r_1)\tfrac{8}{\cos 5^\circ} + \tfrac{4\pi}{3}\cdot 2\cdot 8 + \tfrac{2\pi}{3}(r_1^2 - 4) + 2(24 - 32t)'
+++

A square plate with a part-turned boss on it, a tall arc-shaped wall with a flat end at each end, the boss's outer wall drafted.

The plate is the box −12 ≤ x, y ≤ 12, 0 ≤ z ≤ 4. On it stands the boss: the rectangle 2 ≤ r ≤ 5, 4 ≤ z ≤ 12 of the half-plane φ = 0 (r the distance from the z axis, φ the angle about it from +x toward +y), turned about the z axis from φ = 0 to φ = 240°, and united with the plate. It is a wall 3 thick and 8 tall that runs two thirds of the way round a hollow of radius 2: an outer cylinder of radius 5, an inner cylinder of radius 2, a flat top at z = 12, and two flat end faces in the planes φ = 0 and φ = 240°, each through the z axis, standing on the plate's top.

Draft only the boss's outer cylindrical wall, 5°, with the pull direction +z and the neutral plane z = 4, the plate's top, where the boss stands: the wall becomes a cone through the circle of radius 5 at z = 4, leaning 5° from the vertical, inward as it rises, so that its radius is 5 − (z − 4)·tan 5°. The inner wall, the top, the two end faces and the plate stay as they are, except that the top and the end faces are cut back to meet the cone: the top is the sector 2 ≤ r ≤ 5 − 8·tan 5° (about 4.3) and each end face the quadrilateral 2 ≤ r ≤ 5 − (z − 4)·tan 5° of its plane.

**As a set:**

$$\{|x|, |y| \le 12,\ 0 \le z \le 4\}\ \cup\ \{\,4 \le z \le 12,\ 2 \le r \le 5 - (z - 4)\tan 5^\circ,\ 0 \le \varphi \le 240^\circ\,\}$$
