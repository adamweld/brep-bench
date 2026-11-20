+++
title = "A smooth loft from a point through a triangle to a point"
tags = ["loft", "smooth", "point-ends", "parabola", "analytic"]
origin = "OCCT thrusection/specific/A3"

[volume]
value = 133.02150202128978
exact = "∫ A(1 − z²/36)² dz over −6 ≤ z ≤ 6 with A = 12√3 the triangle's area, = (16/15)·A·6"
tex = '\tfrac{16}{15}\cdot12\sqrt3\cdot6 = \tfrac{384\sqrt3}{5}'

[area]
value = 173.35215200175095
exact = "each of the three faces is the point (1 − z²/36)·q + (0, 0, z), q running along the side of length 4√3 at distance 2 from the axis: area 4√3·∫(1 − s²)√(36 + 16s²) ds over −1 ≤ s ≤ 1, z = 6s; three of them"
tex = '\tfrac{675\sqrt3}{4}\operatorname{arsinh}\tfrac23 - \tfrac{3\sqrt{39}}{2}'
+++

A smooth loft from a point through a triangle to a second point, the two points on the triangle's axis: the sections are joined by curves, not straight lines, so each side of the triangle swells from the points into a curved face.

The triangle lies in the plane z = 0, with corners (4, 0, 0), (−2, 2√3, 0) and (−2, −2√3, 0), equilateral and centered on the z axis. The points are (0, 0, 6) and (0, 0, −6). The loft is the surface traced by one quadratic Bézier curve for each point T of the triangle's boundary: its control points are $(0, 0, -6)$, $(2T_x,\ 2T_y,\ 0)$ and $(0, 0, 6)$, so it runs from one point to the other and passes through T at its parameter ½. The solid has three faces, one over each side of the triangle, meeting along the three curves through the triangle's corners.

**As a set:**

$$\{\,(x, y, z) : (x, y) \in (1 - z^2/36)\cdot\operatorname{poly}\bigl((4,0),(-2,2\sqrt3),(-2,-2\sqrt3)\bigr),\ |z| \le 6\,\}$$
