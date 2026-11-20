+++
title = "A cube united with a bar that passes through it"
tags = ["boolean", "union", "planar", "coincident"]
origin = "OCCT boolean/bopfuse_simple/F1"

[volume]
value = 1300.0
exact = "10·10·10 + 2·5·3·10"
tex = '10\cdot 10\cdot 10 + 2\cdot 5\cdot 3\cdot 10'

[area]
value = 860.0
exact = "6·10·10 + 2·(30 + 100 + 30 − 30)"
tex = '6\cdot 10\cdot 10 + 2\cdot (30 + 100 + 30 - 30)'
+++

A cube of side 10 between the origin and (10, 10, 10), united with a bar between (−5, 3, 0) and (15, 6, 10): it enters the cube through the face x = 0 and leaves through x = 10, strictly inside the faces across, and is as tall as the cube, so its top lies in the cube's top plane and its bottom in the cube's bottom plane.

**As a set:**

$$[0,10]^3 \cup \bigl([-5,15]\times[3,6]\times[0,10]\bigr)$$
