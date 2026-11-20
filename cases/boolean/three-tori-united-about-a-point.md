+++
title = "Three equal tori about one centre, their axes mutually perpendicular, united"
tags = ["boolean", "union", "torus"]
origin = "OCCT tests/boolean/bopfuse_simple/ZP6"
settled = false
+++

Three equal tori, each of major radius 3 and tube radius 1, with their centres all at the origin.

The first has its axis the z axis (its core circle lies in the plane z = 0), the second its axis the y axis (core circle in y = 0), the third its axis the x axis (core circle in x = 0). Their axes are mutually perpendicular.

Make the union of the three. Any two of the tori meet where their core circles cross: the first and second at (±3, 0, 0), the first and third at (0, ±3, 0), the second and third at (0, 0, ±3). The union is one connected solid with the three rings fused into each other.

**As a set:**

$$\{\,(\rho - 3)^2 + z^2 \le 1\,\} \cup \{\,(\sqrt{x^2+z^2} - 3)^2 + y^2 \le 1\,\} \cup \{\,(\sqrt{y^2+z^2} - 3)^2 + x^2 \le 1\,\}$$
