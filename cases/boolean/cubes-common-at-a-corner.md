+++
title = "The common part of two cubes overlapping at a corner"
tags = ["boolean", "common", "analytic", "box"]
origin = "OCCT fclasses/bug7287_4"

[volume]
value = 64.0
exact = "4³"
tex = '4^3'

[area]
value = 96.0
exact = "6·4²"
tex = '6\cdot 4^2'
+++

A cube of side 10 from the origin to (10, 10, 10), and a cube of side 6 from (6, 6, 6) to (12, 12, 12). The small cube's corner (6, 6, 6) lies inside the large one and the large cube's corner (10, 10, 10) inside the small one, so the two overlap in the cube from (6, 6, 6) to (10, 10, 10), of side 4. Make the part common to both: the cube of side 4.

**As a set:**

$$[0,10]^3\cap[6,12]^3$$
