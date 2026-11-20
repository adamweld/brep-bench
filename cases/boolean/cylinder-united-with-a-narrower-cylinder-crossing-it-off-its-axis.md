+++
title = "A cylinder united with a narrower cylinder crossing it at right angles, its axis passing 1 beside the wide one's"
tags = ["boolean", "union", "analytic", "cylinder", "ssi"]
origin = "OCCT tests/boolean/bopfuse_simple/ZE7"

[volume]
value = 1289.3698147494545
exact = "V = πR²·14 + πr²·16 − C, R = 5, r = 3, d = 1, C = ∫ from d−r to d+r of 4√((R² − y²)(r² − (y − d)²)) dy: at each y the wide cylinder takes x from −√(R² − y²) to √(R² − y²), the narrow one z from −√(r² − (y − d)²) to √(r² − (y − d)²)"
tex = '\pi R^2\cdot 14 + \pi r^2\cdot 16 - \int_{d-r}^{d+r} 4\sqrt{(R^2-y^2)\,(r^2-(y-d)^2)}\,dy,\quad R = 5,\ r = 3,\ d = 1'

[area]
value = 728.8606589063598
exact = "area = (2πR·14 + 2πR²) + (2πr·16 + 2πr²) − W − N: the wide wall loses W = ∫ 2R√(r² − (R sin φ − d)²) dφ over the φ with |R sin φ − d| ≤ r, and the narrow wall N = ∫₀^2π 2r√(R² − (d + r sin θ)²) dθ"
tex = '2\pi R\cdot 14 + 2\pi R^2 + 2\pi r\cdot 16 + 2\pi r^2 - \int 2R\sqrt{r^2-(R\sin\varphi-d)^2}\,d\varphi - \int_0^{2\pi} 2r\sqrt{R^2-(d+r\sin\theta)^2}\,d\theta'
+++

A cylinder of radius 5 and length 14 along the z axis, from z = −7 to z = 7, and a cylinder of radius 3 and length 16 parallel to the x axis, from x = −8 to x = 8, its axis the line y = 1, z = 0. The narrow cylinder crosses the wide one at right angles, its axis passing 1 beside the wide one's axis (they do not meet), and goes right through it, out of both sides. Make their union.

The narrow cylinder's wall lies wholly within the wide cylinder's length and, since 1 + 3 < 5, never reaches the far side of the wide cylinder's wall; its end discs and the wide one's end discs are all clear of the other cylinder.

**As a set:**

$$\{\,\rho \le 5,\ |z| \le 7\,\} \cup \{\,(y-1)^2 + z^2 \le 3^2,\ |x| \le 8\,\}$$
