+++
title = "A cube and a frustum standing through it and reaching below it, their common part"
tags = ["boolean", "common", "analytic", "cone", "planar"]
origin = "OCCT tests/boolean/bopcommon_simple/ZF8"

[volume]
value = 128.28170002158322
exact = "πh/3·(a² + ab + b²), h = 10, a = 5/2, b = 3/2"
tex = '\tfrac{\pi h}{3}(a^2 + ab + b^2),\quad h = 10,\ a = \tfrac52,\ b = \tfrac32'

[area]
value = 152.99399923873145
exact = "π(a² + b²) + π(a + b)·√(h² + (a − b)²)"
tex = '\pi(a^2 + b^2) + \pi(a+b)\sqrt{h^2 + (a-b)^2}'
+++

A cube of edge 10: x and y from −5 to 5, z from 0 to 10. A frustum of a cone with its axis the z axis, from z = −5 to z = 10, of radius 3 at z = −5 and radius 3/2 at z = 10 (its radius falls by 1/10 for every unit of rise): it is longer than the cube, its top disc level with the cube's top face and its base end below the cube's bottom face. Take the part common to the two.

The frustum's radius is under 3 everywhere, so it lies well inside the cube's sides and touches them nowhere. The cube's bottom face z = 0 cuts it across, where its radius is 5/2.

What is left is the part of the frustum from z = 0 to z = 10: a frustum of radius 5/2 at the bottom and 3/2 at the top, with both ends flat.

**As a set:**

$$\bigl([-5,5]\times[-5,5]\times[0,10]\bigr) \cap \{\,\rho \le \tfrac{5}{2} - \tfrac{z}{10},\ -5 \le z \le 10\,\}$$
