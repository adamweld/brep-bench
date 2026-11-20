+++
title = "A cylinder with a blind hole flush with its top"
tags = ["boolean", "cut", "analytic", "coincident", "hole"]
origin = "OCCT tests/boolean/bopcut_simple/ZC9"

[volume]
value = 351.85837720205683
exact = "π·4²·8 − π·2²·4"
tex = '128\pi - 16\pi'

[area]
value = 351.85837720205683
exact = "cylinder 96π, less the top disc's hole π·2² and plus the hole's wall 2π·2·4 and its floor π·2²"
tex = '96\pi - 4\pi + 16\pi + 4\pi'
+++

A cylinder of radius 4 and height 8 standing on the xy plane with its axis the z axis, and a second cylinder of radius 2 and height 4 on the same axis from z = 4 to z = 8, so that its top disc lies in the first's top disc. Cut the second away from the first: a hole of radius 2 and depth 4 bored coaxially into the top.

**As a set:**

$$\{\rho \le 4,\ 0 \le z \le 8\} \setminus \{\rho \le 2,\ 4 \le z \le 8\}$$
