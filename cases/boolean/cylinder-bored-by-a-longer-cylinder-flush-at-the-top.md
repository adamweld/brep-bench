+++
title = "A cylinder bored through by a longer cylinder, flush at the top"
tags = ["boolean", "cut", "analytic", "coincident", "hole"]
origin = "OCCT tests/boolean/bopcut_simple/ZD3"

[volume]
value = 301.59289474462014
exact = "π(4² − 2²)·8"
tex = '\pi\left(4^2 - 2^2\right)\cdot 8'

[area]
value = 376.9911184307752
exact = "2π(4 + 2)·8 + 2π(4² − 2²)"
tex = '2\pi(4+2)\cdot 8 + 2\pi\left(4^2-2^2\right)'
+++

A cylinder of radius 4 and height 8 standing on the xy plane with its axis the z axis, and a second cylinder of radius 2 on the same axis from z = −4 to z = 8: longer than the first, its top disc in the first's top disc and its base 4 below the first's base. Cut the second away from the first, leaving a tube of inner radius 2 and outer radius 4, 8 high.

**As a set:**

$$\{\,2 \le \rho \le 4,\ 0 \le z \le 8\,\}$$
