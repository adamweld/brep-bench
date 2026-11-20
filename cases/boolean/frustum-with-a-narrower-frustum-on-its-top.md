+++
title = "A frustum with a much narrower frustum standing on its top"
tags = ["boolean", "union", "analytic", "cone"]
origin = "OCCT tests/boolean/bopfuse_simple/ZM1"

[volume]
value = 409.4542425178697
exact = "πh/3·(R² + Rr + r²) for each: 126π (h = 6, 6 → 3) and 13π/3 (h = 4, 3/2 → 1/2)"
tex = '126\pi + \tfrac{13\pi}{3}'

[area]
value = 350.6647185292911
exact = "36π + π·9·3√5 + (9 − 9/4)π + π·2·√17 + π/4: the lower frustum's base and wall, its top ring, the upper frustum's wall and top"
tex = '36\pi + 27\sqrt5\,\pi + \tfrac{27}{4}\pi + 2\sqrt{17}\,\pi + \tfrac{\pi}{4}'
+++

A frustum of a cone on the z axis, from z = 0 to z = 6: radius 6 at z = 0, narrowing in a straight line to radius 3 at z = 6.

A second frustum, coaxial with it, standing on its top face: from z = 6 to z = 10, radius 3/2 at z = 6, narrowing in a straight line to radius 1/2 at z = 10. Make the union of the two.

The upper frustum's base lies in the lower one's top face, wholly inside its outline, leaving a ring of that face (from radius 3/2 to radius 3) showing.

**As a set:**

$$\{\,0 \le z \le 6,\ \rho \le 6 - \tfrac12 z\,\} \cup \{\,6 \le z \le 10,\ \rho \le \tfrac32 - \tfrac14 (z - 6)\,\}$$
