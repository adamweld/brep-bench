+++
title = "Two equal cylinders overlapping end to end, their common part"
tags = ["tangent", "boolean", "common", "analytic", "coincident", "cylinder"]
origin = "OCCT boolean/bcommon_simple/J1"

[volume]
value = 314.1592653589793
exact = "πr²·4 = 100π, r = 5"
tex = '4\pi r^2 = 100\pi,\quad r = 5'

[area]
value = 282.7433388230814
exact = "2πr·4 + 2πr² = 90π, r = 5"
tex = '2\pi r\cdot 4 + 2\pi r^2 = 90\pi'
+++

Two cylinders of radius 5 on the z axis, the first from z = 0 to z = 10 and the second from z = 6 to z = 16. Make the part common to both.

Between z = 6 and z = 10 their walls coincide. The common part is the cylinder of radius 5 from z = 6 to z = 10: the second's bottom disc and the first's top disc are its ends, and its wall is the part of both walls between them. If the cylinders you make have a seam, turn the second a quarter turn about the z axis, so that the coincident walls are not split in the same place.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cap \{\,\rho \le 5,\ 6 \le z \le 16\,\}$$
