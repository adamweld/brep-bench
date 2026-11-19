+++
title = "A post grooved by a ring centered on its wall"
tags = ["boolean", "cut", "analytic", "torus", "cylinder", "revolution"]
origin = "OCCT tests/boolean/bopcut_simple/ZL2"

[volume]
value = 1153.8078423629372
exact = "πR²H − (π²ar² − 4πr³/3) = 432π − 24π² + 32π/3 = 1328π/3 − 24π², R = a = 6, H = 12, r = 2: the inner half of the torus's tube, by Pappus its half disk of area πr²/2 turned about the axis at a − 4r/(3π)"
tex = '\pi R^2 H - \left(\pi^2 a r^2 - \tfrac{4}{3}\pi r^3\right) = \tfrac{1328\pi}{3} - 24\pi^2,\quad R = a = 6,\ H = 12,\ r = 2'

[area]
value = 714.3925889717932
exact = "2πR(H − 2r) + 2πR² + 2π²ar − 4πr² = 96π + 72π + 24π² − 16π = 152π + 24π²: the wall above and below the groove, the two ends, and the groove, the inner half of the torus, by Pappus its half circle of length πr turned about the axis at a − 2r/π"
tex = '2\pi R (H - 2r) + 2\pi R^2 + 2\pi^2 a r - 4\pi r^2 = 152\pi + 24\pi^2'
+++

A cylinder (a post) of radius 6 on the z axis, from z = 0 to z = 12, less a solid torus about the z
axis whose middle circle, of radius 6, lies in the plane z = 6 and whose tube has radius 2.

The torus's middle circle lies on the post's wall, so the wall cuts the torus's tube through the
middle: the inner half of the tube is inside the post and the outer half outside. The cut leaves a
groove of half-round section around the post, 2 deep, from z = 4 to z = 8, its floor the torus's
inner equator at radius 4. The groove meets the wall at right angles, along the circles of radius
6 at z = 4 and z = 8, where the torus's surface is level.

**As a set:**

$$\{\,\rho \le 6,\ 0 \le z \le 12\,\} \setminus \{\,(\rho - 6)^2 + (z - 6)^2 \le 2^2\,\}$$
