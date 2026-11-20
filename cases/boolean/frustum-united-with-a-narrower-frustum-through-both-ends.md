+++
title = "A frustum united with a longer, narrower frustum that comes out of both ends"
tags = ["boolean", "union", "analytic", "cone"]
origin = "OCCT tests/boolean/bopfuse_simple/ZM6"

[volume]
value = 308.95600252647125
exact = "the larger frustum 78π, plus the two stubs, each πh/3·(R² + Rr + r²): below h = 3, r from 5/2 to 17/8; above h = 3, r from 11/8 to 1"
tex = '78\pi + \pi\Bigl(\bigl(\tfrac{25}{4} + \tfrac{85}{16} + \tfrac{289}{64}\bigr) + \bigl(\tfrac{121}{64} + \tfrac{11}{8} + 1\bigr)\Bigr)'

[area]
value = 307.7648806051062
exact = "(25 − 289/64)π + (4 − 121/64)π + 21√5π + 25π/4 + π + 21√65π/8: the larger frustum's base ring, top ring and wall, the stubs' end discs, and their walls 111√65π/64 and 57√65π/64 (slant √(9 + (3/8)²) = 3√65/8)"
tex = '\bigl(25 - \tfrac{289}{64}\bigr)\pi + \bigl(4 - \tfrac{121}{64}\bigr)\pi + 21\sqrt5\,\pi + \tfrac{25}{4}\pi + \pi + \tfrac{21\sqrt{65}}{8}\pi'
+++

A frustum of a cone on the z axis, from z = 0 to z = 6: radius 5 at z = 0, narrowing in a straight line to radius 2 at z = 6.

A second frustum, coaxial with it, narrower and longer: from z = −3 to z = 9, radius 5/2 at z = −3, narrowing in a straight line to radius 1 at z = 9. So it comes out 3 below the first frustum's base and 3 above its top. Make the union of the two.

Inside the first frustum the second is well clear of its cone surface (radius 17/8 against 5 at z = 0, 11/8 against 2 at z = 6). The cone surface of the second crosses the first one's base plane in a circle of radius 17/8 and its top plane in a circle of radius 11/8. The union is the first frustum with a conical stub on each end.

**As a set:**

$$\{\,0 \le z \le 6,\ \rho \le 5 - \tfrac12 z\,\} \cup \{\,-3 \le z \le 9,\ \rho \le \tfrac52 - \tfrac18 (z + 3)\,\}$$
