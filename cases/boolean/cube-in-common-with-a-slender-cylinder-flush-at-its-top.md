+++
title = "A cube and a slender cylinder longer than it, level with its top, their common part"
tags = ["boolean", "common", "analytic", "cylinder", "coincident"]
origin = "OCCT tests/boolean/bopcommon_simple/ZN3"

[volume]
value = 125.66370614359173
exact = "πr²h with r = 2, h = 10: the common part is the stretch of the cylinder from z = 0 to z = 10"
tex = '\pi r^2 h = 40\pi'

[area]
value = 150.79644737231008
exact = "2πr² + 2πrh = 8π + 40π"
tex = '48\pi'
+++

A cube of side 10 between the origin and (10, 10, 10), and a cylinder of radius 2 on the vertical line through (5, 5), from z = −3 to z = 10: its top disc is in the cube's top face and it extends 3 below the cube's base. Take the part common to the two.

The cylinder is well inside the cube's footprint, so its wall touches nothing; the cube's base plane cuts it at z = 0, and its top disc lies in the cube's top face. The common part is a cylinder of radius 2 and height 10, from z = 0 to z = 10.

**As a set:**

$$[0,10]^3 \cap \{(x-5)^2 + (y-5)^2 \le 2^2,\ -3 \le z \le 10\}$$
