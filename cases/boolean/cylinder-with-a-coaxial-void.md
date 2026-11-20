+++
title = "A cylinder with a void inside it"
tags = ["boolean", "cut", "analytic", "void"]
origin = "OCCT tests/boolean/bopcut_simple/ZD1"

[volume]
value = 351.85837720205683
exact = "π·4²·8 − π·2²·4"
tex = '128\pi - 16\pi'

[area]
value = 376.9911184307752
exact = "cylinder 96π plus the void's wall 2π·2·4 and its two discs 2π·2²"
tex = '96\pi + 16\pi + 8\pi'
+++

A cylinder of radius 4 and height 8 standing on the xy plane with its axis the z axis, and a second cylinder of radius 2 and height 4 on the same axis from z = 2 to z = 6, wholly inside the first and touching none of its faces. Cut the second away from the first: what is left is the first cylinder with a closed void, a second shell, inside it.

**As a set:**

$$\{\rho \le 4,\ 0 \le z \le 8\} \setminus \{\rho \le 2,\ 2 \le z \le 6\}$$
