+++
title = "Three quarters of a cone frustum grown by 1, its edges kept sharp"
tags = ["offset", "grow", "sharp-edge", "cone", "planar", "concave"]
origin = "OCCT tests/offset/shape_type_i/F6"
settled = false
+++

Three quarters of a cone frustum: the part of the cone frustum about the z axis with a base of
radius 10 at z = 0 and a top of radius 6 at z = 8 (its side leans in 1 for every 2 it rises), with φ
from 0° to 270° (φ measured from +x toward +y): all but the quarter with x ≥ 0 and y ≤ 0. Its faces
are the conical side, a flat bottom and a flat top, and two flat cut faces, in the half-plane y = 0,
x ≥ 0 and in the half-plane x = 0, y ≤ 0, which meet along the z axis in a concave edge.

Grow it by 1 with its edges kept sharp: move each face 1 outward along its normal, and extend the
moved faces until they meet, so that no round is added and every edge stays where its two faces
meet.

The side moves out to the cone ρ + z/2 = 10 + √5/2, the bottom down to z = −1 and the top up to z =
9. The cut faces move out 1, the first to the plane y = −1 (for x ≥ 1) and the second to the plane x
= 1 (for y ≤ −1), and are extended until they meet each other along the line x = 1, y = −1, so the
missing quarter shrinks to the part with x ≥ 1 and y ≤ −1.

**As a set:**

$$\{\, \rho \le 10 + \tfrac{\sqrt5}{2} - \tfrac z2,\ -1 \le z \le 9 \,\} \setminus \{\, x > 1,\ y < -1 \,\}$$

Its volume and area are not worked out in closed form here, so the case is unsettled until two kernels agree.
