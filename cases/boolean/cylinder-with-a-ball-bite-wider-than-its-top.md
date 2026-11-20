+++
title = "A cylinder with a bite from its top by a ball wider than it"
tags = ["boolean", "cut", "analytic", "sphere"]
origin = "OCCT tests/boolean/bopcut_simple/ZI8"

[volume]
value = 74.3934568569927
exact = "π·3²·6 less the part of the ball inside the cylinder, the integral from 0 to 3 of 2πρ√(16 − ρ²) dρ = (2π/3)(64 − 7√7)"
tex = '54\pi - \tfrac{2\pi}{3}\left(R^3 - (R^2 - r^2)^{3/2}\right),\quad R = 4,\ r = 3'

[area]
value = 125.53641397748893
exact = "the wall 2π·3·(6 − √7), the ball crosses it √(4² − 3²) = √7 below the top; the base π·3²; the spherical cap in the cylinder, of height 4 − √7, 2π·4·(4 − √7); the top disc is gone"
tex = '2\pi r\left(h - \sqrt{R^2 - r^2}\right) + \pi r^2 + 2\pi R\left(R - \sqrt{R^2 - r^2}\right) = 77\pi - 14\sqrt7\,\pi'
+++

A cylinder of radius 3 and height 6 standing on the xy plane with its axis the z axis, and a ball of radius 4 centred at (0, 0, 6), the middle of the cylinder's top disc. Cut the ball away from the cylinder.

The ball is wider than the cylinder: its surface crosses the cylinder's wall, in a circle √7 below the top, and reaches 4 below the top along the axis, short of the base. The cut takes the whole top disc and leaves a dish bounded by a cap of the ball's surface.

**As a set:**

$$\{\rho \le 3,\ 0 \le z \le 6\} \setminus B_4\bigl((0, 0, 6)\bigr)$$
