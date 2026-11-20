+++
title = "A post united with a narrower post of its own height, hidden inside it"
tags = ["boolean", "union", "analytic", "cylinder", "coincident", "face-contact"]
origin = "OCCT tests/boolean/bopfuse_simple/ZD2"

[volume]
value = 785.3981633974483
exact = "the post is unchanged: π·5²·10 = 250π"
tex = '\pi\cdot 5^2\cdot 10 = 250\pi'

[area]
value = 471.23889803846896
exact = "2π·5² + 2π·5·10 = 150π"
tex = '2\pi\cdot 5^2 + 2\pi\cdot 5\cdot 10 = 150\pi'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A second cylinder of radius 3 with the same axis and the same ends, from z = 0 to z = 10: it lies inside the post, its two end discs in the post's end faces. Make the union of the two.

The narrow cylinder adds nothing; its wall is inside the post and touches nothing, and its end discs lie in the post's end faces. What is left is the post, whole.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \{\,\rho \le 3,\ 0 \le z \le 10\,\}$$
