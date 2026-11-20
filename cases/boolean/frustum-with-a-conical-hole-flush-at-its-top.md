+++
title = "A cone frustum with a conical hole, flush at its top"
tags = ["boolean", "cut", "analytic", "coincident", "cone"]
origin = "OCCT tests/boolean/bopcut_simple/ZM4"

[volume]
value = 579.1584234534517
exact = "πh(A² + AB + B²)/3 with h = 10, A = 6, B = 3, less the same with h = 10, a = 7/6, b = 2 (the hole's radii at z = 0 and z = 10)"
tex = '\tfrac{\pi h}{3}\left(A^2 + AB + B^2\right) - \tfrac{\pi h}{3}\left(a^2 + ab + b^2\right),\quad A = 6,\ B = 3,\ a = \tfrac76,\ b = 2,\ h = 10'

[area]
value = 519.5505536767242
exact = "π(A + B)√(h² + (A − B)²) + π(A² − a²) + π(B² − b²) + π(a + b)√(h² + (b − a)²)"
tex = '\pi(A + B)\sqrt{h^2 + (A - B)^2} + \pi(A^2 - a^2) + \pi(B^2 - b^2) + \pi(a + b)\sqrt{h^2 + (b - a)^2}'
+++

A cone frustum standing on the xy plane, its axis the z axis, from z = 0 to z = 10: radius 6 at its base, radius 3 at its top. And a narrower frustum on the same axis from z = −2 to z = 10: radius 1 at its lower end, radius 2 at its upper end. Cut the narrower from the wider.

The narrower frustum's upper end disc lies in the wider one's top face, leaving a rim 1 wide there, and its lower end is 2 below the base, so the hole comes out through the base as a disc of radius 7/6. The result is a frustum with a conical bore, a ring at each end.

**As a set:**

$$\{\rho \le 6 - \tfrac{3}{10}z,\ 0 \le z \le 10\} \setminus \{\rho \le 1 + \tfrac{1}{12}(z + 2),\ z \le 10\}$$
