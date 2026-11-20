+++
title = "A stepped disc with a pin through its step, tangent to the step's top rim"
tags = ["boolean", "union", "analytic", "revolve", "cone", "cylinder", "tangent"]
origin = "OCCT boolean/bfuse_simple/E3"

[volume]
value = 956.09136424249374
exact = "200π + (4π/3)(6² + 6·4 + 4²) + π·1²·3"
tex = '200\pi + \tfrac{4\pi}{3}(6^2 + 6\cdot 4 + 4^2) + \pi\cdot 1^2\cdot 3'

[area]
value = 850.4962343321078
exact = "100π + 40π + 64π + 10π·2√5 + 16π + 2π·1·3"
tex = '100\pi + 40\pi + 64\pi + 10\pi\cdot 2\sqrt5 + 16\pi + 2\pi\cdot 1\cdot 3'
+++

A body turned about the z axis from two stacked pieces: a disc of radius 10 from z = 0 to z = 2, and on it a cone frustum from z = 2 to z = 6 whose radius falls from 6 at the bottom to 4 at the top. And a thin cylinder of radius 1 from z = 1 to z = 9, its axis the vertical line through (3, 0). Unite them.

The cylinder's axis is off the body's, and its wall reaches x = 4: it touches the rim of the frustum's top, the circle of radius 4, at the single point (4, 0, 6), from inside. Below the top the cylinder lies wholly inside the frustum, whose radius there is at least 4, and its bottom end is inside the disc. The union is the body with the cylinder standing 3 high out of its top, tangent to the top's rim.

**As a set:**

$$\{\,\rho \le 10,\ 0 \le z \le 2\,\} \cup \{\,\rho \le 6 - \tfrac{z-2}{2},\ 2 \le z \le 6\,\} \cup \{\,(x-3)^2 + y^2 \le 1,\ 1 \le z \le 9\,\}$$
