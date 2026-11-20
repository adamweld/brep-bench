+++
title = "A post and a wider ball centred on its top disc, their common part"
tags = ["boolean", "common", "analytic", "cylinder", "sphere"]
origin = "OCCT tests/boolean/bopcommon_simple/ZI8"

[volume]
value = 127.75810124598493
exact = "∫₀³ 2πρ√(25 − ρ²) dρ = (2π/3)(125 − 64)"
tex = '\int_0^3 2\pi\rho\sqrt{25-\rho^2}\,d\rho = \tfrac{2\pi}{3}(125 - 64)'

[area]
value = 135.0884841043611
exact = "π·3² + 2π·3·4 + 2π·5·1: the top disc, the wall to depth 4, and the spherical zone of height 1"
tex = '\pi 3^2 + 2\pi \cdot 3 \cdot 4 + 2\pi \cdot 5 \cdot 1'
+++

A post: a solid cylinder of radius 3 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A ball of radius 5 centred at (0, 0, 10), the middle of the post's top disc. Take the part common to the two.

The ball's surface crosses the post's wall in the circle at z = 6, 4 below the top (3² + 4² = 5²), and the post's base is never reached. What is left is a short post of radius 3 whose top is flat and whose bottom is a spherical dome bulging downward, from the circle at z = 6 to the ball's lowest point (0, 0, 5) on the axis.

**As a set:**

$$\{\,\rho \le 3,\ 0 \le z \le 10\,\} \cap B_5\bigl((0,0,10)\bigr)$$
