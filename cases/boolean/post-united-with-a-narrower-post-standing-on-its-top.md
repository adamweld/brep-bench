+++
title = "A post united with a narrower post standing on its top"
tags = ["boolean", "union", "analytic", "cylinder", "coincident", "face-contact"]
origin = "OCCT tests/boolean/bopfuse_simple/ZC8"

[volume]
value = 955.0441666912972
exact = "π·5²·10 + π·3²·6 = 304π"
tex = '25\pi\cdot 10 + 9\pi\cdot 6 = 304\pi'

[area]
value = 584.3362335677016
exact = "area = 25π (lower base) + 2π·5·10 (lower wall) + (25 − 9)π (the lower top, a ring) + 2π·3·6 (upper wall) + 9π (upper top) = 186π"
tex = '25\pi + 100\pi + 16\pi + 36\pi + 9\pi = 186\pi'
+++

A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A second cylinder of radius 3 with the same axis, from z = 10 to z = 16, standing on the post's top face. Make the union of the two.

They touch over the narrow cylinder's base disc, which lies wholly inside the post's top face; nothing overlaps. What is left is a stepped post: a wide cylinder with a narrow one on it.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \{\,\rho \le 3,\ 10 \le z \le 16\,\}$$
