+++
title = "A closed ruled loft round a diamond-shaped loop"
tags = ["loft", "ruled", "closed", "circle", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestLoft.py::testClosedAdditiveLoftCase"

[volume]
value = 9424.77796076938
exact = "∫ over z of the area between the diamonds |x| + |y| = 30 ± √(25 − z²), 2[(30 + w)² − (30 − w)²] = 240w with w = √(25 − z²): 240·25π/2 = 3000π"
tex = '240\int_{-5}^{5}\sqrt{25 - z^2}\,dz = 3000\pi'

[area]
value = 4584.237346833254
exact = "each of four pieces is a cylinder over the ellipse of the circle's section, cut off by the axes: 4∫₀^{2π} (30 + 5cos θ)√2 · 5√(cos²θ + ½ sin²θ) dθ; the cos θ part integrates to 0, leaving 600√2·4E(1/√2), E the complete elliptic integral of the second kind, E(1/√2) = π^{3/2}/Γ(1/4)² + Γ(1/4)²/(8√π): 2400√2·E(1/√2) ≈ 4584.2373468"
tex = '2400\sqrt{2}\,E\!\left(\tfrac{1}{\sqrt{2}}\right),\quad E\!\left(\tfrac{1}{\sqrt{2}}\right) = \tfrac{\pi^{3/2}}{\Gamma(1/4)^2} + \tfrac{\Gamma(1/4)^2}{8\sqrt{\pi}}'
+++

A closed ruled loft through four circles, each in a plane perpendicular to the next, forming a loop.

Four circles of radius 5. For k = 0, 1, 2, 3 let u_k = (cos 90°k, sin 90°k, 0). The circle k is centred at
30·u_k, in the vertical plane that contains u_k and the z axis direction (so the plane y = 0 for k = 0 and 2,
the plane x = 0 for k = 1 and 3: each circle lies across the loop's path and in a plane at a right angle to
its neighbours'). Its points are 30·u_k + 5(cos θ·u_k + sin θ·(0, 0, 1)), for the angle θ.

The loft is **ruled** and **closed**: the point of circle k at angle θ is joined by a straight line to the
point of circle k + 1 at the same angle θ (circle 3 to circle 0 closing the loop), and the solid is bounded
by these four ruled surfaces, enclosing the loop. There are no end caps: the tube returns to where it began.

Every point of the loop's surface between circles k and k + 1 is
(30 + 5 cos θ)·((1 − s)·u_k + s·u_{k+1}) + 5 sin θ·(0, 0, 1), for s from 0 to 1. So a point (x, y, z) is in
the solid exactly when (|x| + |y| − 30)² + z² ≤ 25: a torus whose "distance from the axis" is |x| + |y|, the
diamond with corners at (±30, 0), (0, ±30) in place of a circle.

**As a set:**

$$\{\,(|x| + |y| - 30)^2 + z^2 \le 25\,\}$$
