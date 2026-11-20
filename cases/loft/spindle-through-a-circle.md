+++
title = "A spindle: a smooth loft from a point through a circle to a point"
tags = ["loft", "smooth", "point-ends", "revolution", "parabola", "analytic"]
origin = "OCCT thrusection/specific/B2"

[volume]
value = 120.63715789784806
exact = "∫ π·(3(1 − z²/16))² dz over −4 ≤ z ≤ 4, = (16/15)·π·3²·4"
tex = '\tfrac{16}{15}\pi\cdot3^2\cdot4 = \tfrac{192\pi}{5}'

[area]
value = 119.58830868413280
exact = "2π∫ ρ·√(1 + ρ′²) dz with ρ = 3(1 − z²/16), which with z = 4s is 2π·3·∫(1 − s²)√(16 + 36s²) ds over −1 ≤ s ≤ 1"
tex = '\tfrac{14\pi\sqrt{13}}{3} + \tfrac{160\pi}{9}\operatorname{arsinh}\tfrac32'
+++

A smooth loft, not a ruled one, from a point through a circle to a second point, both points on the circle's axis: the sections are joined by a curve, not by a straight line, so that the solid swells from each point to the circle.

The circle has radius 3, lies in the plane z = 0 and is centered on the z axis. The points are (0, 0, 4) and (0, 0, −4). The loft is the surface traced by one quadratic Bézier curve for each point of the circle: for the point $C = (3\cos\theta,\ 3\sin\theta,\ 0)$ the curve has control points $(0, 0, -4)$, $(2C_x,\ 2C_y,\ 0)$ and $(0, 0, 4)$. So it runs from one point to the other and passes through $C$ at its parameter ½. Every such curve is a parabola, ρ = 3(1 − z²/16), and the solid is the one it turns about the z axis: pointed at both ends, its one face a surface of revolution.

**As a set:**

$$\{\,\rho \le 3\,(1 - z^2/16),\ |z| \le 4\,\}$$
