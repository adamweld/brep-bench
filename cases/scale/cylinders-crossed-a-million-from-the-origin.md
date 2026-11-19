+++
title = "Two cylinders crossed a million from the origin"
tags = ["boolean", "common", "cylinder", "analytic", "ssi", "far-from-origin", "scale"]
origin = "brep-conformance 1.0.0, booleans/scale/far-from-origin-equal-cylinders"

[volume]
value = 144
exact = "16r³/3, r = 3"
tex = '\tfrac{16}{3}r^3,\quad r = 3'

[area]
value = 144
exact = "16r², r = 3"
tex = '16r^2,\quad r = 3'
+++

The solid common to two cylinders of radius 3 and length 20 that cross at right angles a million
from the origin: a Steinmetz bicylinder. Both are centered on c = (600000, −800000, 0); the axis
of the first runs along (2, 3, 6)/7, the axis of the second along (3, −6, 2)/7, so each runs from
c − 10u to c + 10u along its own direction u, and the two axes meet at c square to each other.
Neither cylinder's end is reached.

The two walls cross in two ellipses whose planes meet along a line through c, and the ellipses
cross each other at two points on that line, where the intersection curve is singular. The solid
is a few units across with every coordinate about 10⁶, along directions no axis lines up with.

**As a set:**

$$\{\, \|q\|^2 - (q\cdot u)^2 \le 9,\ |q\cdot u| \le 10 \,\} \cap \{\, \|q\|^2 - (q\cdot v)^2 \le 9,\ |q\cdot v| \le 10 \,\},\quad q = p - c,\ u = \tfrac{(2, 3, 6)}{7},\ v = \tfrac{(3, -6, 2)}{7}$$
