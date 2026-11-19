+++
title = "A cube with a small cube standing against its side"
tags = ["boolean", "fuse", "analytic", "coincident", "box"]
origin = "OCCT heal/bug33171_2"

[volume]
value = 1064.0
exact = "10³ + 4³"
tex = '10^3 + 4^3'

[area]
value = 664.0
exact = "6·10² − 4² + (6·4² − 4²)"
tex = '6\cdot 10^2 - 4^2 + (6\cdot 4^2 - 4^2)'
+++

A cube of side 10 from the origin to (10, 10, 10), and a cube of side 4 from (10, 3, 3) to (14, 7, 7). The small cube's face at x = 10 lies in the large cube's face at x = 10, wholly inside it with a margin of 3 on every side. Unite them.

**As a set:**

$$[0,10]^3\cup\bigl([10,14]\times[3,7]^2\bigr)$$
