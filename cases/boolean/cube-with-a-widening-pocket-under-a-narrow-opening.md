+++
title = "A cube with a conical pocket that widens below a narrow opening"
tags = ["boolean", "cut", "analytic", "coincident", "pocket"]
origin = "OCCT tests/boolean/bopcut_simple/ZF6"

[volume]
value = 194.00885142487144
exact = "6³ less the frustum, (π·3/3)(2² + 2·1 + 1²) = 7π"
tex = '216 - \tfrac{\pi h}{3}\left(R^2 + Rr + r^2\right) = 216 - 7\pi'

[area]
value = 255.2285427581577
exact = "216, less the opening π·1², plus the pocket's conical wall π(2 + 1)√(3² + 1²) and its floor π·2²"
tex = '216 - \pi + 3\pi\sqrt{10} + 4\pi'
+++

A cube of edge 6 from (−3, −3, 0) to (3, 3, 6), and a cone frustum inside it on the z axis, from z = 3 up to z = 6: base radius 2 at z = 3, top radius 1 at z = 6, so that its small top disc lies in the cube's top face and its base is inside the cube. Cut the frustum away from the cube: a pocket that opens in a circle of radius 1 in the top face and widens as it goes down, to a flat floor of radius 2 at z = 3.

**As a set:**

$$[-3,3]\times[-3,3]\times[0,6] \setminus \{\rho \le 1 + (6-z)/3,\ 3 \le z \le 6\}$$
