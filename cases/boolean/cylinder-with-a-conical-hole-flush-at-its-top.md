+++
title = "A cylinder with a conical hole, flush at its top"
tags = ["boolean", "cut", "analytic", "coincident", "cone"]
origin = "OCCT tests/boolean/bopcut_simple/ZJ7"

[volume]
value = 451.286376648806
exact = "250π − πh(a² + ab + b²)/3 with h = 10, a = 32/13, b = 4 (the cone's radii at z = 0 and z = 10)"
tex = '250\pi - \tfrac{\pi h}{3}\left(a^2 + ab + b^2\right),\quad h = 10,\ a = \tfrac{32}{13},\ b = 4'

[area]
value = 607.32144540876
exact = "π(5² − a²) + π(5² − b²) + 2π·5·10 + π(a + b)·√(h² + (b − a)²)"
tex = '\pi(5^2 - a^2) + \pi(5^2 - b^2) + 2\pi\cdot 5\cdot 10 + \pi(a + b)\sqrt{h^2 + (b - a)^2}'
+++

A cylinder of radius 5 and height 10 standing on the xy plane, its axis the z axis, and a cone frustum on the same axis from z = −3 to z = 10: radius 2 at its lower end (z = −3), radius 4 at its upper end (z = 10). Cut the frustum away from the cylinder.

The frustum's upper end disc lies in the cylinder's top face, so the hole opens at the top as a disc of radius 4, leaving a rim 1 wide, and widens upward. The frustum's lower end is 3 below the cylinder's base, so the hole comes out through the base as a disc of radius 32/13 (the frustum's radius at z = 0). The result is a cylinder with a conical bore and a ring at each end.

**As a set:**

$$\{\rho \le 5,\ 0 \le z \le 10\} \setminus \{\rho \le 2 + \tfrac{2}{13}(z + 3),\ z \le 10\}$$
