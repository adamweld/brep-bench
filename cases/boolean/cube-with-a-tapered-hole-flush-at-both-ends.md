+++
title = "A cube with a tapered hole through it, the cutter flush at both ends"
tags = ["boolean", "cut", "analytic", "coincident", "hole"]
origin = "OCCT tests/boolean/bopcut_simple/ZF7"

[volume]
value = 172.0177028497429
exact = "6³ less the frustum, (π·6/3)(2² + 2·1 + 1²) = 14π"
tex = '216 - \tfrac{\pi h}{3}\left(R^2 + Rr + r^2\right) = 216 - 14\pi'

[area]
value = 257.62072296819946
exact = "216, less the openings π·2² and π·1², plus the hole's conical wall π(2 + 1)√(6² + 1²)"
tex = '216 - 5\pi + 3\pi\sqrt{37}'
+++

A cube of edge 6 from (−3, −3, 0) to (3, 3, 6), and a cone frustum on the z axis as tall as the cube, from z = 0 to z = 6: base radius 2 at z = 0, top radius 1 at z = 6, so that its end discs lie in the cube's bottom and top faces. Cut the frustum away from the cube: a tapered hole through the cube, 2 wide at the bottom and 1 at the top.

**As a set:**

$$[-3,3]\times[-3,3]\times[0,6] \setminus \{\rho \le 2 - z/6\}$$
