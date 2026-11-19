+++
title = "A square frame extruded with a taper, its outside flaring and its hole closing"
tags = ["feature", "extrude", "taper", "draft", "planar", "analytic"]
origin = "FreeCAD src/Mod/Part/TestPartApp.py testExtrudeTaperInverted"

[volume]
value = 14400.0
exact = "h/3·(A₁ + A₂ + √(A₁A₂)) for each frustum, h = 10: 10/3·(40² + 44² + 40·44) − 10/3·(20² + 16² + 20·16) = 17653⅓ − 3253⅓"
tex = '\tfrac{10}{3}\left(40^2 + 44^2 + 40\cdot 44\right) - \tfrac{10}{3}\left(20^2 + 16^2 + 20\cdot 16\right) = 14400'

[area]
value = 5327.529366524536
exact = "(40² − 20²) + (44² − 16²) + 4·(40 + 44)/2·2√26 + 4·(20 + 16)/2·2√26 = 2880 + 480√26 (each wall √(10² + 2²) = 2√26 along its slope)"
tex = '\left(40^2 - 20^2\right) + \left(44^2 - 16^2\right) + 4\cdot\tfrac{40 + 44}{2}\cdot 2\sqrt{26} + 4\cdot\tfrac{20 + 16}{2}\cdot 2\sqrt{26} = 2880 + 480\sqrt{26}'
+++

A square frame extruded with a taper. The frame is the square |x| ≤ 20, |y| ≤ 20 less the square
hole |x| ≤ 10, |y| ≤ 10, in the plane z = 0. It is extruded up to z = 10, and every side face
leans away from the frame's material by the same angle, arctan(1/5), moving 1 sideways for every 5
it rises.

So the outside grows, to the square |x|, |y| ≤ 22 at z = 10. The hole's walls also lean away from
the material, toward the z axis, so the hole closes, to the square |x|, |y| ≤ 8 at z = 10. The
solid is the frustum of a square pyramid from 40 × 40 at z = 0 to 44 × 44 at z = 10, less the
frustum from 20 × 20 to 16 × 16. Its walls are eight flat trapezoids, all with sharp corners: four
outside, from 40 wide to 44, and four inside, from 20 to 16.

**As a set:**

$$\{\,\max(|x|, |y|) \le 20 + z/5,\ 0 \le z \le 10\,\} \setminus \{\,\max(|x|, |y|) \le 10 - z/5\,\}$$
