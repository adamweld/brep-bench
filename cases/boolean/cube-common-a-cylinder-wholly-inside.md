+++
title = "A cube in common with a cylinder wholly inside it"
tags = ["boolean", "common", "cylinder", "contained"]
origin = "OCCT tests/boolean/bopcommon_simple/ZD1, OCCT tests/boolean/bopcommon_simple/ZN4"

[volume]
value = 113.09733552923256
exact = "πr²h, r = 3, h = 4: the cylinder, whole"
tex = '\pi\cdot 3^2\cdot 4 = 36\pi'

[area]
value = 131.94689145077132
exact = "2πrh + 2πr² = 24π + 18π"
tex = '2\pi\cdot 3\cdot 4 + 2\pi\cdot 3^2 = 42\pi'
+++

A cube of side 10 spanning x, y and z from 0 to 10, and a cylinder of radius 3 with its axis the line
x = 5, y = 5, from z = 3 to z = 7. The cylinder lies wholly inside the cube and touches none of its
faces (its wall is 2 from the nearest side faces, its ends 3 from the bottom and top). Make the part
common to both: the cylinder, whole.

**As a set:**

$$\bigl([0,10]^3\bigr) \cap \{\,(x-5)^2 + (y-5)^2 \le 9,\ 3 \le z \le 7\,\}$$
