+++
title = "A block less the hollow block whose hollow it fills"
tags = ["boolean", "cut", "analytic", "coincident", "void"]
origin = "OCCT tests/boolean/boptuc_simple/ZP4"

[volume]
value = 100.0
exact = "the filler itself, 4 × 5 × 5: it shares only its surface with the hollow block"
tex = '4\cdot 5\cdot 5'

[area]
value = 130.0
exact = "2(4·5 + 4·5 + 5·5)"
tex = '2(20 + 20 + 25)'
+++

A hollow block: the cube between the origin and (10, 10, 10) with a box cavity inside it, the box from (2, 3, 4) to (6, 8, 9), closed on all sides.

A filler: that box itself, from (2, 3, 4) to (6, 8, 9), exactly the cavity's size and place, so that each of its six faces lies over the cavity wall it faces. Cut the hollow block away from the filler.

The filler and the hollow block share only the cavity's surface and no volume; the result is the filler as it was.

**As a set:**

$$[2, 6] \times [3, 8] \times [4, 9] \setminus \bigl([0, 10]^3 \setminus (2, 6) \times (3, 8) \times (4, 9)\bigr)$$
