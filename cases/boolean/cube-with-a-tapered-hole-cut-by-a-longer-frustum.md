+++
title = "A cube with a tapered hole cut by a frustum that runs out past its base"
tags = ["boolean", "cut", "analytic", "coincident", "hole"]
origin = "OCCT tests/boolean/bopcut_simple/ZF8"

[volume]
value = 154.73894325499904
exact = "6³ less the part of the frustum inside the cube, radius 1 at the top z = 6 growing by 1/4 per unit down to 5/2 at z = 0: (π·6/3)(5²/4 + 5/2 + 1) = 39π/2"
tex = '216 - \tfrac{\pi h}{3}\left(R^2 + Rr + r^2\right) = 216 - \tfrac{39\pi}{2}'

[area]
value = 261.2273245644038
exact = "216, less the openings π·1² at the top and π·(5/2)² at the bottom, plus the hole's conical wall π(5/2 + 1)√(6² + (3/2)²)"
tex = '216 - \pi - \tfrac{25\pi}{4} + \tfrac{7\pi}{2}\sqrt{36 + \tfrac94}'
+++

A cube of edge 6 from (−3, −3, 0) to (3, 3, 6), and a cone frustum on the z axis from z = −2 to z = 6, longer than the cube: base radius 3 at z = −2, top radius 1 at z = 6, so its top disc lies in the cube's top face and its base end lies 2 below the cube's bottom face. Cut the frustum away from the cube: a tapered hole through the cube, of radius 1 at the top and 5/2 at the bottom.

**As a set:**

$$[-3,3]\times[-3,3]\times[0,6] \setminus \{\rho \le 1 + (6-z)/4,\ -2 \le z \le 6\}$$
