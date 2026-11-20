+++
title = "Two equal cylinders crossing at sixty degrees, united"
tags = ["boolean", "union", "analytic", "cylinder", "ssi"]
origin = "OCCT tests/boolean/bopfuse_simple/ZE2"

[volume]
value = 625.4044711780157
exact = "V = 2πr²L − 16r³/(3 sin θ), r = 3, L = 14, θ = 60°: the common part of two equal cylinders whose axes cross at the angle θ is 16r³/(3 sin θ)"
tex = '2\pi r^2 L - \frac{16 r^3}{3\sin\theta},\quad r = 3,\ L = 14,\ \theta = 60^\circ'

[area]
value = 474.6080238057056
exact = "area = 2(2πrL + 2πr²) − 16r²/sin θ: each wall loses the strip inside the other cylinder, of area 8r²/sin θ"
tex = '2\left(2\pi r L + 2\pi r^2\right) - \frac{16 r^2}{\sin\theta},\quad r = 3,\ L = 14,\ \theta = 60^\circ'
+++

Two cylinders of radius 3 and length 14, whose axes cross at their midpoints, at an angle of 60°. The first lies along the z axis, from z = −7 to z = 7. The second lies along the line through the origin in the direction (√3/2, 0, 1/2), with its ends 7 from the origin along that line, on either side. Make their union.

The cylinders are the same size and the angle between their axes is 60°, so their walls meet in two ellipses. Each cylinder is long enough that its end discs lie wholly outside the other.

**As a set:**

$$\{\,\rho \le 3,\ |z| \le 7\,\} \cup \{\,\|p\|^2 - (p\cdot d)^2 \le 3^2,\ |p\cdot d| \le 7\,\},\quad d = (\tfrac{\sqrt3}{2}, 0, \tfrac12)$$
