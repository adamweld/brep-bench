+++
title = "A cylinder pierced through by a cone frustum"
tags = ["boolean", "cut", "analytic", "cone"]
origin = "OCCT tests/boolean/bopcut_simple/ZJ9"

[volume]
value = 497.31197992540467
exact = "250π − πh(a² + ab + b²)/3 with h = 10, a = 16/7, b = 26/7 (the cone's radii at z = 0 and z = 10)"
tex = '250\pi - \tfrac{\pi h}{3}\left(a^2 + ab + b^2\right),\quad h = 10,\ a = \tfrac{16}{7},\ b = \tfrac{26}{7}'

[area]
value = 601.893792262691
exact = "π(5² − a²) + π(5² − b²) + 2π·5·10 + π(a + b)·√(h² + (b − a)²)"
tex = '\pi(5^2 - a^2) + \pi(5^2 - b^2) + 2\pi\cdot 5\cdot 10 + \pi(a + b)\sqrt{h^2 + (b - a)^2}'
+++

A cylinder of radius 5 and height 10 standing on the xy plane, its axis the z axis, and a cone frustum on the same axis from z = −2 to z = 12: radius 2 at its lower end, radius 4 at its upper end. Cut the frustum away from the cylinder.

The frustum is longer than the cylinder at both ends and narrower than it throughout, so it pierces both end faces: the result is a cylinder with a conical bore, 16/7 in radius at the base and 26/7 at the top, and a ring of the cylinder at each end.

**As a set:**

$$\{\rho \le 5,\ 0 \le z \le 10\} \setminus \{\rho \le 2 + \tfrac{1}{7}(z + 2),\ -2 \le z \le 12\}$$
