+++
title = "A cube united with a block that leaves it past an edge"
tags = ["boolean", "union", "planar", "coincident"]
origin = "OCCT boolean/bopfuse_simple/E9"

[volume]
value = 1500.0
exact = "10·10·10 + (10·15 − 5·10)·5"
tex = '10\cdot 10\cdot 10 + (10\cdot 15 - 5\cdot 10)\cdot 5'

[area]
value = 900.0
exact = "200 + 5·60 + 5·40 + 100 + 100"
tex = '200 + 5\cdot 60 + 5\cdot 40 + 100 + 100'
+++

A cube of side 10 between the origin and (10, 10, 10), united with a block between (5, 0, 0) and (15, 15, 5): it starts inside the cube, leaves it through the faces x = 10 and y = 10 together, past their common edge, and lies in the cube's bottom plane z = 0 and its side plane y = 0.

**As a set:**

$$[0,10]^3 \cup \bigl([5,15]\times[0,15]\times[0,5]\bigr)$$
