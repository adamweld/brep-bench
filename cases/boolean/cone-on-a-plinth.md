+++
title = "A cone on a plinth"
tags = ["boolean", "union", "cone", "cylinder", "analytic"]
origin = "OCCT boolean/gdml_public/A6"

[volume]
value = 360.2359576116296
exact = "π·6²·2 + (1/3)π·4²·8 = 72π + 128π/3"
tex = '\pi\cdot 6^2\cdot 2 + \tfrac13\pi\cdot 4^2\cdot 8 = \tfrac{344\pi}{3}'

[area]
value = 363.7244479838351
exact = "the plinth's bottom 36π and side 2π·6·2, its top ring π(6² − 4²), and the cone's side π·4·√(4² + 8²): 80π + 16√5π"
tex = '36\pi + 24\pi + 20\pi + 16\sqrt5\,\pi = 80\pi + 16\sqrt5\,\pi'
+++

A cone standing on a disk, united. The disk is a cylinder of radius 6 on the z axis from z = 0 to z = 2. The cone stands on it with its base the disk of radius 4 in the plane z = 2, centered on the axis, and its apex at (0, 0, 10). The two are fused into one solid.

**As a set:**

$$\{\,\rho \le 6,\ 0 \le z \le 2\,\} \cup \{\,\rho \le \tfrac{10 - z}{2},\ 2 \le z \le 10\,\}$$
