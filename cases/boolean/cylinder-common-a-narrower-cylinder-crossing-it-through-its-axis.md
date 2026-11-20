+++
title = "A cylinder and a narrower cylinder crossing it at right angles, their common part"
tags = ["boolean", "common", "analytic", "cylinder"]
origin = "OCCT tests/boolean/bopcommon_simple/ZE3"

[volume]
value = 97.2825592430655
exact = "(8/3)·R³·((1 + k²)E(k) − (1 − k²)K(k)), R = 4, r = 2, k = r/R = 1/2, with E and K the complete elliptic integrals of modulus k (parameter k²); it is 4∫ √((R² − y²)(r² − y²)) dy over −r ≤ y ≤ r"
tex = '\tfrac{8}{3} R^3\bigl((1+k^2)E(k) - (1-k^2)K(k)\bigr),\quad R = 4,\ r = 2,\ k = \tfrac{r}{R}'

[area]
value = 119.92071013116079
exact = "8rR·E(k) + 8R²·(E(k) − (1 − k²)K(k)): the narrow cylinder's wall inside the wide one plus the wide one's wall inside the narrow one"
tex = '8 r R\,E(k) + 8R^2\bigl(E(k) - (1-k^2)K(k)\bigr)'
+++

A cylinder of radius 4 and length 12 along the z axis, from z = −6 to z = 6. A second cylinder of radius 2 and the same length along the x axis, from x = −6 to x = 6. Their axes cross at right angles at the midpoint of each, the origin. Take the part common to the two.

The narrow cylinder passes completely through the wide one, out of its wall on both sides, and the wide one's ends lie beyond the narrow one. The surfaces meet in two closed curves, one where the narrow cylinder enters the wall and one where it leaves, which do not touch.

**As a set:**

$$\{\,\rho \le 4,\ |z| \le 6\,\} \cap \{\,y^2 + z^2 \le 2^2,\ |x| \le 6\,\}$$
