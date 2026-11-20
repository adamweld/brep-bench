+++
title = "A cube united with a frustum as tall as it, its axis on a side face, its base circle tangent to the two neighbouring faces"
tags = ["boolean", "union", "analytic", "cone", "coincident", "tangent-point"]
origin = "OCCT tests/boolean/bopfuse_simple/ZG5"

[volume]
value = 256.8407044966673
exact = "V = 6³ + ½·(π·6/3)(3² + 3·1 + 1²) = 216 + 13π: the cube and the half of the frustum on y < 0"
tex = '216 + \tfrac12\cdot\tfrac{\pi\cdot 6}{3}\,(9 + 3 + 1) = 216 + 13\pi'

[area]
value = 247.44631633113337
exact = "area = (36 + 9π/2) (bottom: the cube's face and the half base disc) + (36 + π/2) (top, with the half top disc) + (36 − 24) (face y = 0, less the trapezoid the half frustum covers) + 36 + 36 (faces x = 0, 6) + 36 (face y = 6) + ½π(3 + 1)·√(6² + 2²) (the half cone's wall) = 192 + 5π + 4π√10"
tex = '36 + \tfrac{9\pi}{2} + 36 + \tfrac{\pi}{2} + 12 + 108 + 2\pi\cdot 2\sqrt{10}'
+++

A cube of edge 6 from (0, 0, 0) to (6, 6, 6), and a cone frustum as tall as the cube on the vertical axis through (3, 0), the middle of the cube's side face y = 0: base radius 3 at z = 0, top radius 1 at z = 6. Make their union.

The frustum's base circle, of radius 3 about (3, 0), touches the planes of the cube's faces x = 0 and x = 6 at (0, 0, 0) and (6, 0, 0), the ends of the face y = 0's bottom edge, and nowhere else; higher up the frustum is narrower. Half of the frustum, the half on y < 0, lies outside the cube, joined to it over the trapezoid the frustum's section makes in the face y = 0; the other half is inside the cube and goes.

**As a set:**

$$[0,6]^3 \cup \{(x-3)^2 + y^2 \le (3 - z/3)^2,\ 0 \le z \le 6\}$$
