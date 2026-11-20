+++
title = "A cylinder drilled across by a thinner cylinder through its axis"
tags = ["boolean", "cut", "analytic", "elliptic", "hole"]
origin = "OCCT tests/boolean/bopcut_simple/ZE3"

[volume]
value = 405.37226533130143
exact = "π·4²·10 less the common part of the two cylinders, 8∫ from 0 to 2 of √((2² − z²)(4² − z²)) dz = (8R/3)[(R² + r²)E(k) − (R² − r²)K(k)] with R = 4, r = 2, k = r/R = 1/2, E and K the complete elliptic integrals of modulus k"
tex = '160\pi - \tfrac{8R}{3}\left[(R^2 + r^2)E(k) - (R^2 - r^2)K(k)\right],\quad R = 4,\ r = 2,\ k = \tfrac12'

[area]
value = 419.7728298663427
exact = "the thick cylinder's own 2π·4·10 + 2π·4², less its wall inside the thin cylinder, 8[R²E(k) − (R² − r²)K(k)], plus the thin cylinder's wall inside the thick one, 8rR·E(k); E and K as above, of modulus k = 1/2"
tex = '2\pi R\cdot 10 + 2\pi R^2 - 8\left[R^2E(k) - (R^2-r^2)K(k)\right] + 8rR\,E(k)'
+++

A cylinder of radius 4 and length 10 on the x axis, from x = −5 to x = 5, and a second cylinder of radius 2 and length 10 on the y axis, from y = −5 to y = 5, crossing the first at right angles at both midpoints. Cut the second away from the first: a hole of radius 2 straight through the thick cylinder, through its axis, across its diameter.

The thin cylinder runs 1 past the thick one's wall on each side, so the hole goes right through.

**As a set:**

$$\{y^2 + z^2 \le 16,\ |x| \le 5\} \setminus \{x^2 + z^2 \le 4,\ |y| \le 5\}$$
