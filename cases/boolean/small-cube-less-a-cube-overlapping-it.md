+++
title = "A small cube less the large cube overlapping it"
tags = ["boolean", "cut", "analytic", "box"]
origin = "OCCT fclasses/bug7287_3"

[volume]
value = 152.0
exact = "6³ − 4³"
tex = '6^3 - 4^3'

[area]
value = 216.0
exact = "6·6² − 3·4² + 3·4²"
tex = '6\cdot 6^2 - 3\cdot 4^2 + 3\cdot 4^2'
+++

A cube of side 10 from the origin to (10, 10, 10), and a cube of side 6 from (6, 6, 6) to (12, 12, 12). The small cube's corner (6, 6, 6) lies inside the large one and the large cube's corner (10, 10, 10) inside the small one, so the two overlap in the cube from (6, 6, 6) to (10, 10, 10), of side 4. Cut the large cube from the small one.

**As a set:**

$$[6,12]^3\setminus[0,10]^3$$
