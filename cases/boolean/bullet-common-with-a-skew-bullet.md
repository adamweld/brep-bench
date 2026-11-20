+++
title = "The part two bullets crossing at right angles share, off their axes"
tags = ["boolean", "common", "cylinder", "cone", "revolve", "skew-axes"]
origin = "OCCT modalg_1/bug13211_3"
settled = false
+++

A **bullet** is a solid of revolution: in a plane through its axis, the profile with corners (0, 0),
(6, 0), (6, 8), (3, 14), (0, 14), in (distance from the axis, distance along it), turned a whole
turn about the axis. It is a cylinder of radius 6 and height 8, a cone narrowing from radius 6 to 3
over the next 6, and a flat top of radius 3, 14 long in all.

Bullet A stands on the xy plane: its axis is the vertical line through (0, 1.5), its base the disc
of radius 6 at z = 0, its top at z = 14. Bullet B is a copy of A lying on its side, 14 long from
x = −7 to x = 7 with its axis the horizontal line y = −1.5, z = 7, its base the disc at x = −7 and
its top at x = 7. So B is A turned 180° about the line through (0, 0, 7) in the direction (1, 0, 1),
and the two axes are at right angles and 3 apart, the nearest points of the axes (0, 1.5, 7) and
(0, −1.5, 7).

Make the common part of A and B.

**As a set:** with $r(t) = 6$ for $t \le 8$ and $r(t) = 6 - \tfrac{t - 8}{2}$ for $8 \le t \le 14$,

$$\{\,0 \le z \le 14,\ \sqrt{x^2 + (y - 1.5)^2} \le r(z),\ 0 \le x + 7 \le 14,\ \sqrt{(y + 1.5)^2 + (z - 7)^2} \le r(x + 7)\,\}$$
