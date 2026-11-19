+++
title = "A wedge of a cone grown by 1"
tags = ["offset", "grow", "cone", "apex", "torus", "analytic"]
origin = "OCCT tests/offset/shape_type_i/A5"

[volume]
value = 79.69387852590921
exact = "V + Sd + Md² + 4πd³/3 (Steiner, the wedge being convex), d = 1: V = 4π; S = 8π + 12 (a third of the cone's side 15π and base 9π, and two triangles 3 × 4 / 2); M = 6π + π²/2 + π atan(3/4), half of the side's mean curvature times its area, (cos α/2)·(2π/3)·5 = 4π/3, α = atan(3/4) the half-angle at the apex, and half of each edge's length times the angle its faces' normals turn through: the base's arc 2π by π/2 + α, the two slanting edges 5 by π/2, the axis 4 by π/3, the base's two radii 3 by π/2. In all 58π/3 + 12 + π²/2 + π atan(3/4)"
tex = '\begin{array}{l} V + Sd + Md^2 + \tfrac43\pi d^3 = \tfrac{58\pi}{3} + 12 + \tfrac{\pi^2}{2} + \pi\arctan\tfrac34 \\ d = 1,\ V = 4\pi,\ S = 8\pi + 12,\ M = 6\pi + \tfrac{\pi^2}{2} + \pi\arctan\tfrac34 \end{array}'

[area]
value = 101.31106479916814
exact = "S + 2Md + 4πd² = 24π + 12 + π² + 2π atan(3/4), S and M as above, d = 1"
tex = 'S + 2Md + 4\pi d^2 = 24\pi + 12 + \pi^2 + 2\pi\arctan\tfrac34'
+++

A wedge of a cone: the cone on the z axis with a base of radius 3 at z = 0 and its apex at
(0, 0, 4), the part whose angle about the z axis, φ, measured from +x toward +y, lies between 0°
and 120°. Its faces are the cone's side, its base (a third of the disk of radius 3), and two
triangles in the half-planes at φ = 0° and φ = 120°, with corners at the origin, the apex and
the base's rim, which meet along the z axis. Grow it by 1: the solid of every point within 1 of
the wedge.

Each face moves 1 outward: the side to the cone of the same angle whose apex is 1/sin α = 5/3
higher, α = atan(3/4) being the half-angle at the apex; the base to z = −1; the two triangles 1
out of their half-planes. Each edge becomes a piece of a tube of radius 1 about it, turning
through the angle between its faces' normals: the base's rim, an arc of the circle of radius 3,
a piece of a torus turning through 90° + α; the two slanting edges from the rim to the apex, and
the base's two radii, pieces of cylinders turning through 90°; the edge along the z axis from
the origin to the apex, where the triangles meet at 120°, a piece of a cylinder turning through
60°. Each corner, the apex among them, becomes a piece of a ball of radius 1 about it.

**As a set:**

$$\{\, \rho \le 3\bigl(1 - \tfrac z4\bigr),\ 0 \le z \le 4,\ 0^\circ \le \varphi \le 120^\circ \,\} \oplus B_1$$
