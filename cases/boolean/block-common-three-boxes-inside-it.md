+++
title = "A block in common with three boxes inside it"
tags = ["boolean", "common", "planar", "contained", "multi-solid"]
origin = "OCCT tests/bugs/modalg_1/buc60668"

[volume]
value = 52
exact = "2·2·2 + 3·1·4 + 4·4·2 = 8 + 12 + 32"
tex = '8 + 12 + 32'

[area]
value = 126
exact = "boxes 2·(4 + 4 + 4) = 24, 2·(3 + 12 + 4) = 38, 2·(16 + 8 + 8) = 64"
tex = '24 + 38 + 64'
+++

A block spanning x, y and z from 0 to 10. A second body of three separate boxes, none touching
another: the first from (1, 1, 1) to (3, 3, 3), the second from (4, 1, 1) to (7, 2, 5), the third
from (5, 5, 6) to (9, 9, 8). The three boxes lie strictly inside the block, clear of its faces and of
each other. Make the part common to the block and the body of three boxes: three separate solids,
the three boxes, each whole.

**As a set:**

$$\bigl([0,10]^3\bigr) \cap \bigl([1,3]^3 \cup [4,7]\times[1,2]\times[1,5] \cup [5,9]^2\times[6,8]\bigr)$$
