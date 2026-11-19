+++
title = "A ring seated in a tube's end, their common part"
tags = ["tangent", "boolean", "common", "analytic", "torus", "curve-contact", "revolution"]
origin = "OCCT tests/bugs/modalg_5/bug23876"

[volume]
value = 315.82734083485946
exact = "half the torus, π²ar² = 32π², a = 8, r = 2"
tex = '\pi^2 a r^2 = 32\pi^2,\quad a = 8,\ r = 2'

[area]
value = 516.8892706646062
exact = "half the torus's surface, 2π²ar = 32π², and the flat ring between radius a − r and a + r, 4πar = 64π: 32π² + 64π"
tex = '2\pi^2 a r + 4\pi a r = 32\pi^2 + 64\pi'
+++

A tube about the z axis, every point between 6 and 10 from the axis with 0 ≤ z ≤ 10, made by
turning its rectangular section about the axis; and a solid torus about the z axis whose middle
circle, of radius 8, lies in the plane of the tube's top, z = 10, and whose tube has radius 2. Make
the solid common to both.

The torus fits the tube's wall exactly. Its inner equator, the circle of radius 6 at z = 10, is the
top rim of the tube's bore, and its outer equator, of radius 10, the top rim of the tube's outer
wall; along each rim the torus is tangent to that wall. Its lower half lies within the tube and its
upper half above it. The common part is the lower half of the torus: a half ring whose flat top is
the ring between radius 6 and radius 10 in the plane z = 10, bounded by the tube's two rims. Neither
of the tube's walls is part of the result.

**As a set:**

$$\{\,6 \le \rho \le 10,\ 0 \le z \le 10\,\} \cap \{\,(\rho - 8)^2 + (z - 10)^2 \le 2^2\,\}$$
