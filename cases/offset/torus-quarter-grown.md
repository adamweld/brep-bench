+++
title = "A quarter of a torus grown by 1"
tags = ["offset", "grow", "torus", "analytic"]
origin = "OCCT tests/offset/shape_type_i/C3"

[volume]
value = 879.9146232598195
exact = "(π/2)R·π(r + d)² + 2(πr²d + π²rd²/2 + 2πd³/3), R = 10, r = 3, d = 1: the quarter tube of radius r + d (Pappus), and beyond each end half of the end disk grown by d (2πr²d + π²rd² + 4πd³/3, the slab and the outer half of the tube about its rim) = 80π² + 18π + 3π² + 4π/3 = 83π² + 58π/3"
tex = '\tfrac{\pi}{2}R\cdot\pi(r + d)^2 + 2\left(\pi r^2 d + \tfrac12\pi^2 r d^2 + \tfrac23\pi d^3\right) = 83\pi^2 + \tfrac{58\pi}{3},\quad R = 10,\ r = 3,\ d = 1'

[area]
value = 523.116840829086
exact = "(π/2)R·2π(r + d) + 2(πr² + π²rd + 2πd²), R = 10, r = 3, d = 1 = 40π² + 18π + 6π² + 4π = 46π² + 22π"
tex = '\tfrac{\pi}{2}R\cdot 2\pi(r + d) + 2\left(\pi r^2 + \pi^2 r d + 2\pi d^2\right) = 46\pi^2 + 22\pi'
+++

A quarter of the solid torus about the z axis whose tube, of radius 3, runs round the circle of
radius 10 in the plane z = 0: the part whose angle about the z axis, φ, measured from +x toward
+y, lies between 0° and 90°. Its ends are disks of radius 3 in the half-planes y = 0, x ≥ 0 and
x = 0, y ≥ 0. Grow it by 1: the solid of every point within 1 of it.

The torus moves 1 outward, to the tube of radius 4 about the same circle, still between 0° and
90°. Each end disk moves 1 out of its half-plane, beyond the quarter, and its rim, the circle of
radius 3 where the disk meets the torus at a right angle, becomes a quarter of a torus's tube of
radius 1 about that circle, joining the moved disk to the moved torus. Nothing else meets: the
two ends lie far apart, three quarters of a turn round the axis.

**As a set:**

$$\{\, (\rho - 10)^2 + z^2 \le 3^2,\ 0^\circ \le \varphi \le 90^\circ \,\} \oplus B_1$$
