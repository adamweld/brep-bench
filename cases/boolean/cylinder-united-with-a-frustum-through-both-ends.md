+++
title = "A cylinder united with a longer frustum that comes out of both ends"
tags = ["boolean", "union", "analytic", "cylinder", "cone"]
origin = "OCCT tests/boolean/bopfuse_simple/ZJ9"

[volume]
value = 572.8497854280139
exact = "150π + two stubs: the one below, h = 3, r from 3 to 21/8, and the one above, h = 3, r from 15/8 to 3/2, each πh/3·(R² + Rr + r²)"
tex = '150\pi + \pi\bigl(9 + \tfrac{63}{8} + \tfrac{441}{64}\bigr) + \pi\bigl(\tfrac{225}{64} + \tfrac{45}{16} + \tfrac94\bigr)'

[area]
value = 433.70902347907577
exact = "(25 − 441/64)π + (25 − 225/64)π + 60π + 9π + 9π/4 + 27√65π/8: the cylinder's two end rings and wall, the stubs' end discs, and their walls 135√65π/64 and 81√65π/64 (slant √(9 + (3/8)²) = 3√65/8)"
tex = '\bigl(25 - \tfrac{441}{64}\bigr)\pi + \bigl(25 - \tfrac{225}{64}\bigr)\pi + 60\pi + 9\pi + \tfrac94\pi + \tfrac{27\sqrt{65}}{8}\pi'
+++

A cylinder of radius 5, its axis the z axis, from z = 0 to z = 6.

A frustum of a cone, coaxial with it and longer than it, from z = −3 to z = 9: radius 3 at z = −3, narrowing in a straight line to radius 3/2 at z = 9. So it comes out 3 below the cylinder's base and 3 above its top. Make the union of the two.

Inside the cylinder the frustum is well clear of the wall (its radius there runs from 21/8 at z = 0 to 15/8 at z = 6); the cone surface crosses the cylinder's base plane in a circle of radius 21/8 and its top plane in a circle of radius 15/8. The union is the cylinder with a conical stub on each end.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 6\,\} \cup \{\,-3 \le z \le 9,\ \rho \le 3 - \tfrac18 (z + 3)\,\}$$
