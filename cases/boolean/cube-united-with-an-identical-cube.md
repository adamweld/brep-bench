+++
title = "A cube united with an identical cube"
tags = ["boolean", "union", "analytic", "coincident"]
origin = "OCCT tests/bugs/modalg_6/bug28284_3"

[volume]
value = 1000.0
exact = "the two are the same solid: 10³"
tex = '10^3'

[area]
value = 600.0
exact = "6·10²"
tex = '6\cdot 10^2'
+++

Two cubes of side 10, each between the origin and (10, 10, 10), the one exactly on the other: every face, edge and corner of the one coincides with the same of the other. Unite them. The union is the whole cube.

**As a set:**

$$[0,10]^3 \cup [0,10]^3$$
