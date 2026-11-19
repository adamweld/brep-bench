+++
title = "A square frame extruded with its outside and its hole both flaring"
tags = ["feature", "extrude", "taper", "planar", "analytic"]
origin = "FreeCAD src/Mod/Part/TestPartApp.py testExtrudeTaperSameAsOuter"

[volume]
value = 12800.0
exact = "h/3·(A₁ + A₂ + √(A₁A₂)) for each frustum, h = 10: 10/3·(40² + 44² + 40·44) − 10/3·(20² + 24² + 20·24) = 17653⅓ − 4853⅓"
tex = '\tfrac{10}{3}\left(40^2 + 44^2 + 40\cdot 44\right) - \tfrac{10}{3}\left(20^2 + 24^2 + 20\cdot 24\right) = 12800'

[area]
value = 5170.697990959506
exact = "(40² − 20²) + (44² − 24²) + 4·(40 + 44)/2·2√26 + 4·(20 + 24)/2·2√26 = 2560 + 512√26 (each wall √(10² + 2²) = 2√26 along its slope)"
tex = '\left(40^2 - 20^2\right) + \left(44^2 - 24^2\right) + 4\cdot\tfrac{40 + 44}{2}\cdot 2\sqrt{26} + 4\cdot\tfrac{20 + 24}{2}\cdot 2\sqrt{26} = 2560 + 512\sqrt{26}'
+++

A square frame extruded with a taper that opens its hole as well as its outside. The frame is the
square |x| ≤ 20, |y| ≤ 20 less the square hole |x| ≤ 10, |y| ≤ 10, in the plane z = 0. It is
extruded up to z = 10, and every side face, outside and in, leans outward from the z axis by the
same angle, arctan(1/5), moving 1 sideways for every 5 it rises.

So the outside grows, to the square |x|, |y| ≤ 22 at z = 10, and the hole grows with it, to the
square |x|, |y| ≤ 12. The hole's walls lean into the frame's material, the opposite of a draft,
which leans every wall away from the material and would close the hole. The frame stays 10 wide
all the way up. The solid is the frustum of a square pyramid from 40 × 40 at z = 0 to 44 × 44 at
z = 10, less the frustum from 20 × 20 to 24 × 24. Its walls are eight flat trapezoids, all with
sharp corners.

**As a set:**

$$\{\,\max(|x|, |y|) \le 20 + z/5,\ 0 \le z \le 10\,\} \setminus \{\,\max(|x|, |y|) \le 10 + z/5\,\}$$
