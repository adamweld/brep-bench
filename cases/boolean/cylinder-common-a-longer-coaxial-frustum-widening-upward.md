+++
title = "A cylinder and a longer coaxial frustum, their common part"
tags = ["boolean", "common", "analytic", "cylinder", "cone"]
origin = "OCCT tests/boolean/bopcommon_simple/ZJ7"

[volume]
value = 198.96753472735358
exact = "πh/3·(a² + ab + b²), h = 10, a = 2, b = 3"
tex = '\tfrac{\pi h}{3}(a^2 + ab + b^2),\quad h = 10,\ a = 2,\ b = 3'

[area]
value = 198.70378160069006
exact = "π(a² + b²) + π(a + b)·√(h² + (a − b)²)"
tex = '\pi(a^2 + b^2) + \pi(a+b)\sqrt{h^2 + (a-b)^2}'
+++

A cylinder of radius 4 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A frustum of a cone on the same axis, from z = −5 to z = 10, of radius 3/2 at z = −5 and radius 3 at z = 10 (its radius grows by 1/10 for every unit of rise): it is longer than the cylinder, its wide top disc level with the cylinder's top disc and its narrow end below the cylinder's base. Take the part common to the two.

The frustum's radius never reaches the cylinder's 4, so it touches the cylinder's wall nowhere. The cylinder's base plane z = 0 cuts it across, where its radius is 2.

What is left is the frustum's part from z = 0 to z = 10: radius 2 at the bottom and 3 at the top, both ends flat.

**As a set:**

$$\{\,\rho \le 4,\ 0 \le z \le 10\,\} \cap \{\,\rho \le 2 + \tfrac{z}{10},\ -5 \le z \le 10\,\}$$
