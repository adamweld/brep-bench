+++
title = "A cylinder shelled inward, open at both ends: a tube"
tags = ["shell", "inward", "cylinder", "analytic"]
origin = "FreeCAD src/Mod/Part/TestPartApp.py testIssue2876"

[volume]
value = 2615.3758841135027
exact = "π(R² − (R − t)²)H, R = 10, t = 1.5, H = 30: = 832.5π"
tex = '\pi\left(R^2 - (R - t)^2\right)H,\quad R = 10,\ t = 1.5,\ H = 30:\ = 832.5\pi'

[area]
value = 3661.5262377589042
exact = "2πRH + 2π(R − t)H + 2π(R² − (R − t)²) = 600π + 510π + 55.5π = 1165.5π"
tex = '2\pi R H + 2\pi (R - t) H + 2\pi\left(R^2 - (R - t)^2\right) = 1165.5\pi'
+++

A cylinder of radius 10 on the z axis, from z = 0 to z = 30, shelled inward to a wall 1.5 thick,
open at both its flat ends.

The outside of the wall is the cylinder's side, unmoved, and the inside is the side moved 1.5
inward, the cylinder of radius 8.5 on the same axis. It runs on to both openings' planes, z = 0
and z = 30, where the wall ends in flat rims. So the solid is a tube: everything between 8.5 and 10
from the z axis with 0 ≤ z ≤ 30.

**As a set:**

$$\{\,8.5 \le \rho \le 10,\ 0 \le z \le 30\,\}$$
