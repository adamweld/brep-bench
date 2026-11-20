+++
title = "A cube united with a longer cylinder whose axis is one of its vertical edges"
tags = ["boolean", "union", "analytic", "cylinder", "planar", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/Z9"

[volume]
value = 2526.8140296446395
exact = "V = 1000 + 36π·16 − 9π·10 (the cylinder's volume less its quarter inside the cube)"
tex = '1000 + 36\pi\cdot 16 - 9\pi\cdot 10'

[area]
value = 1158.5840131753953
exact = "area = 600 + 72π + 192π − 18π − 120 − 30π: the cube and the cylinder (two ends, 36π each, and its wall, 2π·6·16), less the cube's top and bottom where the cylinder covers them (9π each), its faces x = 0 and y = 0 where it does (60 each) and the cylinder's wall inside the cube (a quarter of 2π·6·10)"
tex = '600 + 72\pi + 192\pi - 18\pi - 120 - 30\pi'
+++

A cube with x, y and z from 0 to 10.

A cylinder of radius 6, its axis the z axis, which is the cube's vertical edge through the origin, from z = −3 to z = 13: it runs 3 below the cube and 3 above it. Make the union of the cube and the cylinder.

The cylinder's wall crosses the cube's faces x = 0 and y = 0 square, along the lines x = 0, y = 6 and y = 0, x = 6; the quarter of the cylinder inside the cube, a quarter section of 9π, is bounded by those two faces.

**As a set:**

$$\bigl([0,10]^3\bigr) \cup \{\,\rho \le 6,\ -3 \le z \le 13\,\}$$
