+++
title = "A drum with a ring-shaped void turned a full turn inside it"
tags = ["feature", "revolve", "groove", "void", "torus", "analytic"]
origin = "FreeCAD tests/src/Mod/PartDesign/App/Revolution.cpp GrooveTwoSidesThroughAll, GrooveProjectAxis"

[volume]
value = 35922.58305088143
exact = "πR²H − 2π²ar², R = 20, H = 30, a = 10, r = 3: = 12000π − 180π²"
tex = '\pi R^2 H - 2\pi^2 a r^2,\quad R = 20,\ H = 30,\ a = 10,\ r = 3:\ = 12000\pi - 180\pi^2'

[area]
value = 7467.537835310309
exact = "2πR² + 2πRH + 4π²ar: the drum's two ends and wall, and the void's torus: = 2000π + 120π²"
tex = '2\pi R^2 + 2\pi R H + 4\pi^2 a r = 2000\pi + 120\pi^2'
+++

A solid drum with a closed cavity shaped like a ring, cut by turning a disk all the way round the
drum's axis.

The drum is a cylinder of radius 20 on the z axis, from z = −15 to z = 15. The disk has radius 3 and
centre (10, 0, 0), and lies in the plane y = 0. Turned a full turn about the z axis, it sweeps the
torus of major radius 10 and minor radius 3 about the z axis, centred at the origin. That torus is
removed from the drum.

The torus reaches radius 13 and height 3 from the middle, so it stays well inside the drum's wall,
top and floor. The result is the drum with a sealed ring-shaped cavity, a solid bounded by two
shells: the drum's outside, and the cavity's wall, one toroidal face with no ends.

**As a set:**

$$\{\,\rho \le 20,\ |z| \le 15\,\} \setminus \{\,(\rho - 10)^2 + z^2 \le 3^2\,\}$$
