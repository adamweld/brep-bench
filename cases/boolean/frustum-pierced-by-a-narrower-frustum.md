+++
title = "A cone frustum pierced through by a narrower frustum"
tags = ["boolean", "cut", "analytic", "cone"]
origin = "OCCT tests/boolean/bopcut_simple/ZM6"

[volume]
value = 587.7129113858457
exact = "πh(A² + AB + B²)/3 with h = 10, A = 6, B = 3, less the same with h = 10, a = 8/7, b = 13/7 (the hole's radii at z = 0 and z = 10)"
tex = '\tfrac{\pi h}{3}\left(A^2 + AB + B^2\right) - \tfrac{\pi h}{3}\left(a^2 + ab + b^2\right),\quad A = 6,\ B = 3,\ a = \tfrac87,\ b = \tfrac{13}{7},\ h = 10'

[area]
value = 516.1136895679637
exact = "π(A + B)√(h² + (A − B)²) + π(A² − a²) + π(B² − b²) + π(a + b)√(h² + (b − a)²)"
tex = '\pi(A + B)\sqrt{h^2 + (A - B)^2} + \pi(A^2 - a^2) + \pi(B^2 - b^2) + \pi(a + b)\sqrt{h^2 + (b - a)^2}'
+++

A cone frustum standing on the xy plane, its axis the z axis, from z = 0 to z = 10: radius 6 at its base, radius 3 at its top. And a narrower frustum on the same axis from z = −2 to z = 12: radius 1 at its lower end, radius 2 at its upper end. Cut the narrower from the wider.

The narrower frustum is longer at both ends and narrower throughout, so it pierces both end faces: the result is a frustum with a conical bore, 8/7 in radius at the base and 13/7 at the top, and a ring at each end.

**As a set:**

$$\{\rho \le 6 - \tfrac{3}{10}z,\ 0 \le z \le 10\} \setminus \{\rho \le 1 + \tfrac{1}{14}(z + 2),\ -2 \le z \le 12\}$$
