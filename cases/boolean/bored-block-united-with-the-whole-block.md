+++
title = "A block with a blind hole united with the same block without it"
tags = ["boolean", "union", "analytic", "hole", "coincident"]
origin = "OCCT boolean/bfuse_simple/L3"

[volume]
value = 4000
exact = "20·20·10"
tex = '20\cdot 20\cdot 10'

[area]
value = 1600
exact = "2·20² + 4·20·10"
tex = '2\cdot 20^2 + 4\cdot 20\cdot 10'
+++

A block from (0, 0, 0) to (20, 20, 10), and the same block with a blind hole bored up into it from its bottom face: a cylinder of radius 4 about the vertical line through (10, 10), from z = 0 to z = 6, taken out of it. Unite them.

Every face of the holed block lies on a face of the plain block, except the hole's wall and its flat end at z = 6, which lie inside it. The union is the plain block, a box 20 by 20 by 10, the hole filled.

**As a set:**

$$[0,20]^2\times[0,10]$$
