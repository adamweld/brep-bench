+++
title = "A groove turned inside a drum about a tilted axis, sealed in it"
tags = ["feature", "revolve", "groove", "void", "torus", "analytic"]
origin = "FreeCAD tests/src/Mod/PartDesign/App/Revolution.cpp GrooveStartOffset, GrooveProjectAxis"

[volume]
value = 37205.63162302305
exact = "πR²H − (θ/2π)·2π²ar², R = 20, H = 30, θ = 100°, a = 10, r = 3: = 12000π − 50π²"
tex = '\pi R^2 H - \tfrac{\theta}{2\pi}\cdot 2\pi^2 a r^2,\quad R = 20,\ H = 30,\ \theta = 100^\circ,\ a = 10,\ r = 3:\ = 12000\pi - 50\pi^2'

[area]
value = 6668.720788313848
exact = "2πR² + 2πRH + (θ/2π)·4π²ar + 2πr²: the drum, the groove's torus and its two end disks: = 2018π + 100π²/3"
tex = '2\pi R^2 + 2\pi R H + \tfrac{\theta}{2\pi}\cdot 4\pi^2 a r + 2\pi r^2 = 2018\pi + \tfrac{100}{3}\pi^2'
+++

A solid drum with a closed cavity inside it: a groove that stays inside the material all the way
round, cut by turning a disk part of a turn about a tilted axis.

The drum is a cylinder of radius 20 on the z axis, from z = −15 to z = 15.

The groove: let A be the line through the origin along (1/2, 0, √3/2), tilted 30° from the z axis
toward +x. The disk D has radius 3 and centre (0, 10, 0), and lies in the plane that holds A and
that centre, the plane z = √3·x. Turned about A (right-handed about the direction given), its centre
runs round a circle of radius 10 about A. The groove is everything the disk passes through between
the turns of 40° and 140° from where it lies, so it starts 40° round and sweeps 100°: a sector of
the torus of radii 10 and 3 about A, its two ends flat disks of radius 3 in the half-planes through
A at 40° and 140°. It is removed from the drum.

No part of the groove reaches the drum's faces: every point of it is within 13 of the origin. So
the result is the drum with a sealed cavity, a solid bounded by two shells, the drum's outside and
the cavity's wall.

**As a set:**

$$\{\,\rho \le 20,\ |z| \le 15\,\} \setminus \bigcup_{t \in [40^\circ,\, 140^\circ]} R_A(t)\, D$$
