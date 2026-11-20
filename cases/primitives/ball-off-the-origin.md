+++
title = "A ball off the origin"
tags = ["primitive", "centroid", "inertia"]
origin = "OCCT tests/bugs/moddata_2/bug705"

[volume]
value = 523.5987755982989
exact = "4πr³/3, r=5"
tex = '\tfrac{4}{3}\pi r^3,\quad r = 5'

[area]
value = 314.1592653589793
exact = "4πr²"
tex = '4\pi r^2'

[centroid]
value = [3, -2, 4]
exact = "the center"

[inertia]
value = [[5235.987755982989, 0, 0], [0, 5235.987755982989, 0], [0, 0, 5235.987755982989]]
exact = "(2/5)·V·r², about each axis through the center, and no products of inertia (of unit density)"
+++

A solid ball of radius 5 centered on the point (3, −2, 4).

**As a set:**

$$\{\,(x-3)^2 + (y+2)^2 + (z-4)^2 \le 5^2\,\}$$
