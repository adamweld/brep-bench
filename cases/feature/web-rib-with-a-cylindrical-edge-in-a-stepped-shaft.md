+++
title = "A web rib with a cylindrical edge in the corner of a stepped shaft"
tags = ["feature", "rib", "revolve", "union", "cylinder", "analytic"]
origin = "OCCT tests/feat/featrf/A9"

[volume]
value = 54356.24643894275
exact = "π(20²·30 + 14²·20 + 8²·20) + 20·(Q(18) − Q(14)) = 3680π + 20·(Q(18) − Q(14)), where Q(R) = 2√(R² − 4) + R² asin(2/R) is the area of {|y| ≤ 2, x > 0, ρ ≤ R}"
tex = '3680\pi + 20\bigl(Q(18) - Q(14)\bigr),\quad Q(R) = 2\sqrt{R^2 - 4} + R^2 \arcsin\tfrac{2}{R}'

[area]
value = 9208.963228454986
exact = "2880π (the shaft's own area) − 20·28 asin(1/7) (the band of the second cylinder's side under the rib) + 20·36 asin(1/9) (the rib's outer face, a band of the cylinder ρ = 18) + 40(√320 − √192) (its two flat sides, each 20 high and √320 − √192 deep); the rib's foot on the shoulder and its top are each Q(18) − Q(14) and cancel"
tex = '2880\pi - 560\arcsin\tfrac17 + 720\arcsin\tfrac19 + 40\bigl(\sqrt{320} - \sqrt{192}\bigr)'
+++

A stepped shaft of three cylinders about the z axis: radius 20 from z = 0 to 30, radius 14 from
z = 30 to 50, and radius 8 from z = 50 to 70. The ring from ρ = 14 to 20 at z = 30 is the first
shoulder, and the ring from ρ = 8 to 14 at z = 50 the second.

A web rib stands in the corner between the first shoulder and the second cylinder's side. Its
profile, in the half-plane y = 0, x > 0, is the vertical line ρ = 18; turned about the z axis, the
line sweeps the cylinder ρ = 18. The rib is the plate 4 thick, 2 to each side of the plane y = 0 on
the +x side, between the second cylinder's side and that cylinder, from the shoulder at z = 30 up to
z = 50. So it meets the shoulder from ρ = 14 to ρ = 18 and the second cylinder's side from z = 30 to
z = 50; its top lies in the plane z = 50 of the second shoulder, which it continues; its two flat
sides are the planes y = ±2, and its outer face is a piece of the cylinder ρ = 18. The shaft and the
rib are one solid.

**As a set:**

$$\{\rho \le 20,\ 0 \le z \le 30\} \cup \{\rho \le 14,\ 30 \le z \le 50\} \cup \{\rho \le 8,\ 50 \le z \le 70\} \cup \{\,|y| \le 2,\ x > 0,\ 14 \le \rho \le 18,\ 30 \le z \le 50\,\}$$
