+++
title = "A cube with four edges rounded by half its side: a cylinder"
tags = ["fillet", "analytic", "cylinder", "consumed"]
origin = "OCCT tests/bugs/moddata_1/bug119"

[volume]
value = 785.3981633974483
exact = "π·5²·10 = 250π"
tex = '\pi r^2 h = 250\pi,\quad r = 5,\ h = 10'

[area]
value = 471.23889803846896
exact = "2π·5·10 + 2π·5² = 150π"
tex = '2\pi r h + 2\pi r^2 = 150\pi'
+++

A cube of side 10 between the origin and (10, 10, 10). Round its four edges parallel to the z axis,
each to radius 5.

Each round is a quarter cylinder, tangent to the two faces its edge joins, and it takes half of
each of them: the four meet along the middle of each side face, which is consumed, and the result
is a cylinder of radius 5 about the line x = y = 5, with the cube's top and bottom, z = 10 and
z = 0, cut to its discs.

**As a set:**

$$[0,10]^3 \cap \{\,(x-5)^2 + (y-5)^2 \le 5^2\,\}$$
