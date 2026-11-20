+++
title = "A cylinder united with a narrower cylinder crossing it at right angles through its axis"
tags = ["boolean", "union", "analytic", "cylinder", "ssi"]
origin = "OCCT tests/boolean/bopfuse_simple/ZE3"

[volume]
value = 1282.5764433452966
exact = "V = πR²·14 + πr²·16 − C, C = (8R³/3)·[(1 + k²)E(k) − (1 − k²)K(k)], R = 5, r = 3, k = r/R: C is the common part, 8∫₀ʳ √((R² − y²)(r² − y²)) dy, E and K the complete elliptic integrals of the first and second kind (modulus k)"
tex = '\pi R^2\cdot 14 + \pi r^2\cdot 16 - \frac{8R^3}{3}\Bigl[(1+k^2)E(k) - (1-k^2)K(k)\Bigr],\quad R = 5,\ r = 3,\ k = \tfrac{r}{R}'

[area]
value = 725.3539672409217
exact = "area = (2πR·14 + 2πR²) + (2πr·16 + 2πr²) − 8R²[E(k) − (1 − k²)K(k)] − 8rR·E(k): the wide cylinder's wall loses the part inside the narrow one, 8R²[E − (1 − k²)K], and the narrow one's wall the part inside the wide one, 8rR·E"
tex = '2\pi R\cdot 14 + 2\pi R^2 + 2\pi r\cdot 16 + 2\pi r^2 - 8R^2\bigl[E(k) - (1-k^2)K(k)\bigr] - 8rR\,E(k)'
+++

A cylinder of radius 5 and length 14 along the z axis, from z = −7 to z = 7, and a cylinder of radius 3 and length 16 along the x axis, from x = −8 to x = 8: the narrow one crosses the wide one at right angles, its axis meeting the wide one's axis at the origin, and passes right through it, out of both sides. Make their union.

The narrow cylinder's wall lies wholly within the wide cylinder's length, its two end discs are clear of the wide cylinder, and the wide one's end discs, at z = ±7, are clear of the narrow one.

**As a set:**

$$\{\,\rho \le 5,\ |z| \le 7\,\} \cup \{\,y^2 + z^2 \le 3^2,\ |x| \le 8\,\}$$
