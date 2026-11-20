+++
title = "A cube with a conical void inside it"
tags = ["boolean", "cut", "analytic", "void"]
origin = "OCCT tests/boolean/bopcut_simple/ZF9"

[volume]
value = 203.6299789264902
exact = "6³ less the frustum, (π·3/3)((3/2)² + (3/2)(3/4) + (3/4)²) = 63π/16"
tex = '216 - \tfrac{\pi h}{3}\left(R^2 + Rr + r^2\right) = 216 - \tfrac{63\pi}{16}'

[area]
value = 246.69411654273443
exact = "216 plus the void's walls: the cone π(3/2 + 3/4)√(3² + (3/4)²) and the two discs π(3/2)² and π(3/4)²"
tex = '216 + \tfrac{9\pi}{4}\sqrt{9 + \tfrac{9}{16}} + \tfrac{45\pi}{16}'
+++

A cube of edge 6 from (−3, −3, 0) to (3, 3, 6), and a cone frustum inside it on the z axis, from z = 1.5 to z = 4.5: base radius 3/2 at z = 1.5, top radius 3/4 at z = 4.5. It lies wholly inside the cube and touches none of its faces. Cut the frustum away from the cube: what is left is the cube with a closed conical void, a second shell, inside it.

**As a set:**

$$[-3,3]\times[-3,3]\times[0,6] \setminus \{\rho \le 3/2 - (z - 3/2)/4,\ 3/2 \le z \le 9/2\}$$
