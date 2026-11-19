+++
title = "Two cubes united, overlapping at a corner"
tags = ["boolean", "fuse", "analytic", "box"]
origin = "OCCT fclasses/bug7287_5"

[volume]
value = 1152.0
exact = "10³ + 6³ − 4³"
tex = '10^3 + 6^3 - 4^3'

[area]
value = 720.0
exact = "6·10² + 6·6² − 6·4²"
tex = '6\cdot 10^2 + 6\cdot 6^2 - 6\cdot 4^2'
+++

A cube of side 10 from the origin to (10, 10, 10), and a cube of side 6 from (6, 6, 6) to (12, 12, 12). The small cube's corner (6, 6, 6) lies inside the large one and the large cube's corner (10, 10, 10) inside the small one, so the two overlap in the cube from (6, 6, 6) to (10, 10, 10), of side 4. Unite them.

**As a set:**

$$[0,10]^3\cup[6,12]^3$$
