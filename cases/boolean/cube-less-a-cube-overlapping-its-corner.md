+++
title = "A cube less a cube overlapping its corner"
tags = ["boolean", "cut", "analytic", "box"]
origin = "OCCT fclasses/bug7287_2"

[volume]
value = 936.0
exact = "10³ − 4³"
tex = '10^3 - 4^3'

[area]
value = 600.0
exact = "6·10² − 3·4² + 3·4²"
tex = '6\cdot 10^2 - 3\cdot 4^2 + 3\cdot 4^2'
+++

A cube of side 10 from the origin to (10, 10, 10), and a cube of side 6 from (6, 6, 6) to (12, 12, 12). The small cube's corner (6, 6, 6) lies inside the large one and the large cube's corner (10, 10, 10) inside the small one, so the two overlap in the cube from (6, 6, 6) to (10, 10, 10), of side 4. Cut the small cube from the large one.

**As a set:**

$$[0,10]^3\setminus[6,12]^3$$
