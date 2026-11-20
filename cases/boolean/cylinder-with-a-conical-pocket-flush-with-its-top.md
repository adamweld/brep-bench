+++
title = "A cylinder with a conical pocket opening in its top"
tags = ["boolean", "cut", "analytic", "coincident", "pocket"]
origin = "OCCT tests/boolean/bopcut_simple/ZJ5"

[volume]
value = 347.66958699727047
exact = "π·4²·8 less the frustum, (π·4/3)(1² + 1·3 + 3²) = 52π/3"
tex = '128\pi - \tfrac{\pi h}{3}\left(r^2 + rR + R^2\right) = 128\pi - \tfrac{52\pi}{3}'

[area]
value = 332.65867136422764
exact = "cylinder 96π, less the opening π·3², plus the pocket's wall π(1 + 3)√(4² + 2²) and its floor π·1²"
tex = '96\pi - 9\pi + 4\pi\sqrt{20} + \pi'
+++

A cylinder of radius 4 and height 8 standing on the xy plane with its axis the z axis, and a cone frustum on the same axis from z = 4 to z = 8: radius 1 at z = 4 and radius 3 at z = 8, so that it widens upward and its wide top disc lies in the cylinder's top disc. Cut the frustum away from the cylinder: a conical pocket 4 deep, opening in a circle of radius 3 and narrowing to a floor of radius 1.

**As a set:**

$$\{\rho \le 4,\ 0 \le z \le 8\} \setminus \{\rho \le 1 + (z-4)/2,\ 4 \le z \le 8\}$$
