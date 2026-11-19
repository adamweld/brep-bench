+++
title = "A drum united with a ring that lies on its rim"
tags = ["tangent", "boolean", "union", "analytic", "torus", "curve-contact"]
origin = "OCCT tests/bugs/modalg_7/bug31858_1"

[volume]
value = 3763.3777308584226
exact = "πR²H + π²ar² = 1000π + 63π², R = 10, H = 10, a = 7, r = 3: the drum and the half of the torus above its top"
tex = '\pi R^2 H + \pi^2 a r^2 = 1000\pi + 63\pi^2,\quad R = 10,\ H = 10,\ a = 7,\ r = 3'

[area]
value = 1407.2666633801277
exact = "2πRH + πR² + π(a − r)² + 2π²ar = 200π + 100π + 16π + 42π² = 316π + 42π²: the wall, the bottom, the top inside the ring's hole, and the upper half of the torus"
tex = '2\pi R H + \pi R^2 + \pi (a - r)^2 + 2\pi^2 a r = 316\pi + 42\pi^2'
+++

A cylinder (a drum) of radius 10 on the z axis, from z = 0 to z = 10, united with a solid torus
about the z axis whose middle circle, of radius 7, lies in the plane of the drum's top, z = 10, and
whose tube has radius 3.

The torus's outer equator, the circle of radius 10 at z = 10, is the drum's top rim, and along it
the torus touches the drum's wall from inside: both surfaces are upright there, tangent along the
whole rim. The lower half of the torus lies inside the drum, and the upper half stands on the
drum's top over the ring between radius 4 and radius 10. The union is the drum with a half torus on
its top. The wall runs on up into the torus without a crease where the rim was, and the drum's top
is left as the disk of radius 4 inside the ring's hole.

**As a set:**

$$\{\,\rho \le 10,\ 0 \le z \le 10\,\} \cup \{\,(\rho - 7)^2 + (z - 10)^2 \le 3^2\,\}$$
