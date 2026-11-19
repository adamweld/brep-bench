+++
title = "A disk revolved both ways past a half turn: a whole torus"
tags = ["feature", "revolve", "two-sided", "overlap", "torus", "analytic"]
origin = "FreeCAD tests/src/Mod/PartDesign/App/Revolution.cpp TwoSidesOverlappingAngles"

[volume]
value = 2131.8345506353016
exact = "2π²Rr², R = 12, r = 3: = 216π² (the two sweeps together cover the whole turn)"
tex = '2\pi^2 R r^2,\quad R = 12,\ r = 3:\ = 216\pi^2'

[area]
value = 1421.2230337568676
exact = "4π²Rr = 144π²: the torus's face alone, no end faces"
tex = '4\pi^2 R r = 144\pi^2'
+++

A disk revolved two ways at once about the z axis, each way through more than half a turn, so that
the two sweeps overlap and together close into a whole torus.

The disk has radius 3 and centre (12, 0, 0), and lies in the plane y = 0. It is revolved about the z
axis through 200° turning from +x toward +y, and through 200° the other way, from +x toward −y. The
solid is everything the disk passes through in either sweep, one solid.

Each sweep runs past the half turn, so the two overlap between 160° and 200° round from the start
either way, and between them they cover the whole turn. The solid is the torus of major radius 12
and minor radius 3 about the z axis. No end face is left where either sweep stops, since each end
lies inside the other sweep.

**As a set:**

$$\bigcup_{t \in [-200^\circ,\, 200^\circ]} R_z(t)\, D \;=\; \{\,(\rho - 12)^2 + z^2 \le 3^2\,\}$$

D is the disk.
