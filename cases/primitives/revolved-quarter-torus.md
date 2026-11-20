+++
title = "A quarter torus from a disk turned a quarter turn"
tags = ["primitive", "revolve", "torus", "analytic"]
origin = "OCCT tests/bugs/moddata_3/bug25202_4"

[volume]
value = 197.39208802178717
exact = "(π/2)·R·πr² (Pappus), R = 10, r = 2: 20π²"
tex = '\tfrac{\pi}{2}\,R\,\pi r^2 = 20\pi^2,\quad R = 10,\ r = 2'

[area]
value = 222.52482925050552
exact = "(π/2)·R·2πr + 2πr² = 20π² + 8π"
tex = '\tfrac{\pi}{2}\,R\cdot 2\pi r + 2\pi r^2 = 20\pi^2 + 8\pi'
+++

The disk of radius 2 centered on (10, 0, 0) in the plane y = 0, turned a quarter turn about the z
axis, from the plane y = 0 toward +y (right-handed), to the plane x = 0. The solid it sweeps is a
quarter of a torus: its two end faces are the disk in y = 0 and its turn in x = 0, and its
side is toroidal.

**As a set:**

$$\{\,(\rho - 10)^2 + z^2 \le 2^2,\ x \ge 0,\ y \ge 0\,\}$$
