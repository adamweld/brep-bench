+++
title = "A rectangle extruded symmetrically with a taper, flaring both ways"
tags = ["feature", "extrude", "taper", "planar", "analytic"]
origin = "FreeCAD tests/src/Mod/Part/App/FeatureExtrusion.cpp::testExecuteAngledRev"

[volume]
value = 2733.3333333333335
exact = "2·∫₀⁵ (20 + 2t/5)(12 + 2t/5) dt = 2·(1200 + 160 + 20/3) = 8200/3"
tex = '2\int_0^5 \left(20 + \tfrac{2t}{5}\right)\left(12 + \tfrac{2t}{5}\right)dt = \tfrac{8200}{3}'

[area]
value = 1309.4666538486187
exact = "2·22·14 + (4·21 + 4·13)√26 = 616 + 136√26: the two ends, and eight trapezoids of slant height √26 (four 20 to 22 wide, area 21√26; four 12 to 14, area 13√26)"
tex = '2\cdot 22\cdot 14 + 136\sqrt{26} = 616 + 136\sqrt{26}'
+++

The rectangle |x| ≤ 10, |y| ≤ 6 in the plane z = 0, extruded 5 up and 5 down, symmetrically about its own plane. Every side face leans away from the rectangle's middle by the same angle, arctan(1/5): it moves 1 sideways for every 5 it runs from the plane z = 0. The rectangle itself is the narrowest section, a waist at z = 0; the section at height z is |x| ≤ 10 + |z|/5, |y| ≤ 6 + |z|/5, so the two ends at z = ±5 are 22 by 14.

The result has ten faces: the two flat ends and eight flat trapezoids, four above and four below, creased along the rectangle at z = 0.

**As a set:**

$$\{\,|x| \le 10 + |z|/5,\ |y| \le 6 + |z|/5,\ |z| \le 5\,\}$$
