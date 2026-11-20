+++
title = "A cone frustum grown by 1"
tags = ["offset", "grow", "cone", "torus", "analytic"]
origin = "OCCT tests/offset/shape_type_i/A3"

[volume]
value = 265.6763984551612
exact = "V + Sd + Md² + 4πd³/3 (Steiner, the frustum being convex), d = 1: V = 28π; S = 20π + 6π√13 (the two ends 4π and 16π, the side π(4 + 2)·√13); M = 3π + 4π² − 2π·atan(3/2), the side's mean curvature times its area (π·(3/√13)·√13 = 3π) and half of each rim's length times the angle its faces' normals turn through: the base's rim 8π by π − atan(3/2), the top's rim 4π by atan(3/2). In all 4π² + (157/3)π + 6π√13 − 2π·atan(3/2)"
tex = '\begin{array}{l} V + Sd + Md^2 + \tfrac43\pi d^3,\quad d = 1,\ V = 28\pi,\ S = 20\pi + 6\pi\sqrt{13} \\ M = 3\pi + 4\pi^2 - 2\pi\arctan\tfrac32 \end{array}'

[area]
value = 228.81750504745065
exact = "S + 2Md + 4πd², S and M as above, d = 1: 8π² + 30π + 6π√13 − 4π·atan(3/2)"
tex = 'S + 2Md + 4\pi d^2 = 8\pi^2 + 30\pi + 6\pi\sqrt{13} - 4\pi\arctan\tfrac32'
+++

A frustum of a cone on the z axis: its base a disk of radius 4 at z = 0, its top a disk of radius
2 at z = 3, its side the cone between their rims, slanting in by 2 over a height of 3. Grow it by
1: the solid of every point within 1 of the frustum.

Each face moves 1 outward: the base to z = −1, the top to z = 4, and the side to the cone of
the same angle, 1 out along its normal. Each rim becomes a piece of a torus's tube of radius 1 about
the rim's circle, turning through the angle between its faces' normals: the base's rim, of radius
4, where the side meets the base at an inside angle of atan(3/2), through π − atan(3/2); the top's
rim, of radius 2, where the side meets the top at an inside angle of π − atan(3/2), through
atan(3/2).

**As a set:**

$$\{\,\rho \le 4 - \tfrac{2z}{3},\ 0 \le z \le 3\,\} \oplus B_{1}$$
