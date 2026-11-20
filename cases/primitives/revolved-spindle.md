+++
title = "A spindle of two cones turned part of a turn"
tags = ["primitive", "revolve", "analytic", "cone", "partial-turn"]
origin = "OCCT modalg_2/bug22588, OCCT modalg_2/bug22818_1, OCCT modalg_2/bug22818_2"

[volume]
value = 251.32741228718345
exact = "80π = θ·A·x̄ = (4π/3)·30·2: the triangle's area is ½·10·6 = 30 and its centroid is 6/3 = 2 from the axis"
tex = '\theta A\,\bar x = \tfrac{4\pi}{3}\cdot 30\cdot 2 = 80\pi'

[area]
value = 257.24657770891264
exact = "60 + 8π√13 + 24π√2: two triangles of 30, the lower cone (2π/3)·6·√52 = 8π√13, the upper (2π/3)·6·6√2 = 24π√2"
tex = '60 + 8\pi\sqrt{13} + 24\pi\sqrt{2}'
+++

In the xz plane, take the triangle with corners (0, 0, 0), (0, 0, 10) and (6, 0, 4): one side lies
along the z axis, and the third corner is off it. Turn it 240° about the z axis, right-handed, from
+x toward +y. The solid is everything the triangle passes through: a spindle of two cones, a short
one below and a long one above, sharing the circle of radius 6 at z = 4, with a 120° wedge missing.

Its faces are the lower cone's side, the upper cone's side, and the triangle where it starts, in
the plane y = 0 at x ≥ 0, and where it ends, in the plane through the z axis at 240° from +x.

**As a set:**

$$\{\,\tfrac{2}{3}\rho \le z \le 10 - \rho,\ \rho \le 6,\ 0 \le \varphi \le 240^\circ\,\}$$
