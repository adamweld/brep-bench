+++
title = "A cup shelled with its wall centred on its faces, its outer rim rounded"
tags = ["shell", "centred", "round", "cylinder", "torus", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestThickness.py testArcJoinProducesValidCenteredCurvedWall"

[volume]
value = 3133.3448102794273
exact = "Pappus, 2π∫ρ dA over the section: the floor 0 ≤ ρ ≤ 10, |z| ≤ 1, 200π; the wall 10 ≤ ρ ≤ 11, 0 ≤ z ≤ 20, 420π, and 9 ≤ ρ ≤ 10, 1 ≤ z ≤ 20, 361π; the quarter disk of radius 1 at the rim, 2π·(π/4)(10 + 4/(3π)) = 5π² + 2π/3: = 2945π/3 + 5π²"
tex = '200\pi + 420\pi + 361\pi + 2\pi\cdot\tfrac{\pi}{4}\left(10 + \tfrac{4}{3\pi}\right) = \tfrac{2945\pi}{3} + 5\pi^2'

[area]
value = 3255.996660868636
exact = "the bottom π·10², the quarter torus 2π·(π/2)(10 + 2/π) = 10π² + 2π, the outside 2π·11·20, the rim π(11² − 9²), the inside 2π·9·19 and its floor π·9²: = 1005π + 10π²"
tex = '100\pi + \left(10\pi^2 + 2\pi\right) + 440\pi + 40\pi + 342\pi + 81\pi = 1005\pi + 10\pi^2'
+++

A cylinder of radius 10 on the z axis, from z = 0 to z = 20, shelled to a wall 2 thick, open at
its top face, z = 20, with the wall centred on the faces it is grown from: half of it, 1 thick,
inside the side and the bottom, and half outside them.

At the bottom rim, the faces moved outward part, and are joined by a round: the wall's outside
there is the quarter torus of radius 1 about the rim circle. The faces moved inward cross, and are
extended until they meet in a sharp edge. The wall runs on up to the opening's plane, z = 20,
where it ends in a flat rim, the ring between radii 9 and 11.

So the solid is every point within 1 of the cylinder's side or bottom, cut off at z = 20: the
cylinder of radius 11 from z = −1 to z = 20 with its bottom rim rounded to radius 1, less the
cylinder of radius 9 from z = 1 up.

**As a set:**

$$\{\,d(p, S) \le 1,\ z \le 20\,\}, \quad S = \{\,\rho \le 10,\ z = 0\,\} \cup \{\,\rho = 10,\ 0 \le z \le 20\,\}$$

S is the cylinder's bottom and side.
