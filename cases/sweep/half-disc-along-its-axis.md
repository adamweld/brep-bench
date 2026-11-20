+++
title = "A half disc swept along its own axis"
tags = ["sweep", "analytic", "cylinder", "planar"]
origin = "OCCT modalg_1/bug13395"

[volume]
value = 314.1592653589793
exact = "(πr²/2)·h = (25π/2)·8"
tex = '\tfrac{\pi r^2}{2}\,h = 100\pi'

[area]
value = 284.20352248333654
exact = "2·(25π/2) + π·5·8 + 10·8: two half discs, the half cylinder, the flat"
tex = '25\pi + 40\pi + 80'
+++

A half disc in the xy plane: the segment from the origin to (5, 0, 0), turned half a turn about the
z axis from +x toward +y, covers the half disc of radius 5 with y ≥ 0. Sweep that face along the
straight line from the origin to (0, 0, 8): a path along the z axis, the axis the face was turned
about, and so square to its plane. The solid is the half cylinder.

**As a set:**

$$\{\,\rho \le 5,\ y \ge 0,\ 0 \le z \le 8\,\}$$
