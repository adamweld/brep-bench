+++
title = "A cube united with a taller cone frustum passing out through its top and bottom"
tags = ["boolean", "union", "analytic", "cone", "planar"]
origin = "OCCT tests/boolean/bopfuse_simple/ZF8"

[volume]
value = 269.4070751110265
exact = "6³ + (π·3/3)(2.5² + 2.5·2 + 2²) + (π·3/3)(1² + 1·0.5 + 0.5²) = 216 + 15.25π + 1.75π: the cube and the two ends of the frustum outside it"
tex = '216 + \pi\,(6.25 + 5 + 4) + \pi\,(1 + 0.5 + 0.25) = 216 + 17\pi'

[area]
value = 278.04107521653316
exact = "area = (72 − 4π − π) (the cube's top and bottom less the frustum's sections) + 144 (its sides) + π(2.5 + 2)·(√37/2) + π(1 + 0.5)·(√37/2) (the cone's wall outside the cube, below and above) + π·2.5² + π·0.5² (the frustum's two end discs) = 216 + 1.5π + 3π√37"
tex = '72 - 5\pi + 144 + \tfrac{9}{4}\pi\sqrt{37} + \tfrac{3}{4}\pi\sqrt{37} + 6.5\pi'
+++

A cube of edge 6 from (−3, −3, 0) to (3, 3, 6), and a cone frustum on the z axis from z = −3 to z = 9, its radius 2 − z/6 at height z: 2.5 at z = −3, 2 at the cube's bottom face, 1 at its top face, 0.5 at z = 9. Make their union.

The frustum is narrower than the cube everywhere it is inside it, so it crosses the bottom face and the top face in circles (radii 2 and 1) and its wall is clear of the cube's sides. What is left is the cube with a cone frustum running through it, three of its length standing out below and three above.

**As a set:**

$$[-3,3]\times[-3,3]\times[0,6] \cup \{\rho \le 2 - z/6,\ -3 \le z \le 9\}$$
