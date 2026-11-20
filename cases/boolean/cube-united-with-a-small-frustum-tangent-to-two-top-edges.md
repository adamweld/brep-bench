+++
title = "A cube united with a small cone frustum on its top, its base circle tangent to two top edges"
tags = ["boolean", "union", "analytic", "cone", "face-contact", "tangent-point"]
origin = "OCCT tests/boolean/bopfuse_simple/ZG8"

[volume]
value = 228.3700210735098
exact = "6³ + (π·3/3)(1.5² + 1.5·0.75 + 0.75²) = 216 + (63/16)π"
tex = '216 + \pi\,(2.25 + 1.125 + 0.5625) = 216 + \tfrac{63}{16}\pi'

[area]
value = 232.55694960158036
exact = "area = (216 − 2.25π) (the cube, its top less the frustum's base disc) + π(1.5 + 0.75)·√(3² + 0.75²) (the cone's wall, slant 3√17/4) + 0.5625π (the frustum's top) = 216 − (27/16)π + (27√17/16)π"
tex = '216 - \tfrac{9}{4}\pi + \tfrac{9}{4}\pi\cdot\tfrac{3\sqrt{17}}{4} + \tfrac{9}{16}\pi'
+++

A cube of edge 6 from (0, 0, 0) to (6, 6, 6), and a cone frustum standing on its top face: axis the vertical line through (1.5, 1.5), base radius 1.5 at z = 6, top radius 0.75 at z = 9. Make their union.

The base circle, of radius 1.5 about (1.5, 1.5), touches the cube's top edges x = 0 and y = 0 at (0, 1.5) and (1.5, 0): the frustum's base disc lies wholly in the top face and meets its boundary at those two points. Nothing overlaps.

**As a set:**

$$[0,6]^3 \cup \{(x-1.5)^2 + (y-1.5)^2 \le (1.5 - (z-6)/4)^2,\ 6 \le z \le 9\}$$
