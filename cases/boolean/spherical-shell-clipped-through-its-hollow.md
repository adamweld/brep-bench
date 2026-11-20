+++
title = "A spherical shell clipped by a box face just above its centre"
tags = ["boolean", "common", "analytic", "sphere", "planar", "shell"]
origin = "OCCT bugs/modalg_7/bug31470"

[volume]
value = 480.66367599923836
exact = "the shell 4π(6³ − 3³)/3 = 252π, less the outer cap above z = 1, π·5²·(18 − 5)/3 = 325π/3, plus the inner cap, π·2²·(9 − 2)/3 = 28π/3"
tex = '252\pi - \tfrac{325\pi}{3} + \tfrac{28\pi}{3}'

[area]
value = 424.1150082346221
exact = "outer sphere 4π·36 − 2π·6·5 = 84π, inner sphere 4π·9 − 2π·3·2 = 24π, and the flat face z = 1, an annulus π(6² − 1) − π(3² − 1) = 27π"
tex = '84\pi + 24\pi + 27\pi'
+++

A spherical shell, the points between the spheres of radius 3 and 6 about the origin, and a box from (−10, −10, −10) to (10, 10, 1). Take the part of the shell inside the box.

The box is far wider than the shell on every side but the top, where its face z = 1 lies just above the centre and cuts through the hollow as well as the shell's wall: the top is a flat ring, 1 above the centre, between the circle of radius √35 on the outer sphere and the circle of radius √8 on the inner one, and the hollow is open there.

**As a set:**

$$\{\,3 \le \lVert p \rVert \le 6,\ z \le 1\,\}$$
