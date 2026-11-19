+++
title = "A wedge of a cut ball grown by 1"
tags = ["offset", "grow", "sphere", "pole", "torus", "analytic"]
origin = "OCCT tests/offset/shape_type_i/B2"

[volume]
value = 376.96112483654457
exact = "V + Sd + Md² + 4πd³/3 (Steiner, the wedge being convex), d = 1: V = 448π/9; S = 57π + 12 − 25 acos(3/5) (sphere 80π/3, top 16π/3, the two flats 25π + 12 − 25 acos(3/5)); M = 26π/3 + 5π²/4 + (5π/2) asin(3/5) + (4π/3) acos(3/5), the sphere's area over its radius and half of each edge's length times the angle its faces' normals turn through: the two meridians 5(π/2 + asin(3/5)) by π/2, the rim's arc 8π/3 by acos(3/5), the axis 8 by π/3, the top's two radii 4 by π/2"
tex = '\begin{array}{l} V + Sd + Md^2 + \tfrac43\pi d^3,\quad d = 1,\ V = \tfrac{448\pi}{9},\ S = 57\pi + 12 - 25\arccos\tfrac35 \\ M = \tfrac{26\pi}{3} + \tfrac{5\pi^2}{4} + \tfrac{5\pi}{2}\arcsin\tfrac35 + \tfrac{4\pi}{3}\arccos\tfrac35 \end{array}'

[area]
value = 277.4596371159137
exact = "S + 2Md + 4πd², S and M as above, d = 1"
tex = 'S + 2Md + 4\pi d^2'
+++

A wedge of a ball cut flat: the part of the ball of radius 5 centered on the origin with z ≤ 3
whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 120°. Its poles
are on the z axis; it runs from the south pole, (0, 0, −5), up to the plane z = 3. Its faces are
the sphere, the flat top (a third of the disk of radius 4 at z = 3), and two flat faces in the
half-planes at φ = 0° and φ = 120°, which meet along the z axis. Grow it by 1: the solid of every
point within 1 of the wedge.

Each face moves 1 outward: the sphere to the sphere of radius 6, the top to z = 4, the two flats
1 out of their half-planes. Each edge becomes a piece of a tube of radius 1 about it, turning
through the angle between its faces' normals: the two edges where the sphere meets a flat, each an
arc of a great circle from the south pole up to z = 3, become pieces of tori of major
radius 5 turning through 90°; the edge where the sphere meets the top, an arc of the circle of
radius 4 at z = 3, a piece of a torus about the z axis of major radius 4 turning through
acos(3/5) ≈ 53.13°; the edge along the z axis from z = −5 to z = 3, where the flats meet at 120°,
a piece of a cylinder turning through 60°; and the two radii of the top, pieces of cylinders
turning through 90°. Each corner, the south pole among them, becomes a piece of a ball of radius
1 about it.

**As a set:**

$$\{\, \lVert p \rVert \le 5,\ z \le 3,\ 0^\circ \le \varphi \le 120^\circ \,\} \oplus B_1$$
