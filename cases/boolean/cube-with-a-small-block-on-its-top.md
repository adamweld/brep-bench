+++
title = "A cube with a small block standing on its top"
tags = ["boolean", "union", "planar", "coincident"]
origin = "OCCT boolean/bopfuse_simple/A6"

[volume]
value = 1036.0
exact = "10·10·10 + 3·3·4"
tex = '10\cdot 10\cdot 10 + 3\cdot 3\cdot 4'

[area]
value = 648.0
exact = "6·10·10 + 4·3·4"
tex = '6\cdot 10\cdot 10 + 4\cdot 3\cdot 4'
+++

A cube of side 10 between the origin and (10, 10, 10), united with a block between (3, 3, 10) and (6, 6, 14): it stands on the cube's top face, its bottom covering a 3 by 3 patch well inside that face, and the two share no volume.

**As a set:**

$$[0,10]^3 \cup \bigl([3,6]^2\times[10,14]\bigr)$$
