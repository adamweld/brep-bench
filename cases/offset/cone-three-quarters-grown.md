+++
title = "Three quarters of a cone grown by 1"
tags = ["offset", "grow", "cone", "apex", "planar", "reentrant"]
origin = "OCCT tests/offset/shape_type_i/A7"
settled = false
+++

Three quarters of a cone: the cone on the z axis with a base of radius 6 at z = 0 and its apex at
(0, 0, 8), the part whose angle about the z axis, φ, measured from +x toward +y, lies between 0°
and 270°, so everything but the quarter with x ≥ 0 and y ≤ 0. Its faces are the cone's side, its
base (three quarters of the disk of radius 6), and two triangles with corners at the origin, the
apex and the base's rim, in the half-planes y = 0, x ≥ 0 and x = 0, y ≤ 0, which meet along the z
axis in a reentrant edge. Grow it by 1: the solid of every point within 1 of it.

The base moves down to z = −1, the side 1 out to a cone of the same angle, with radius
6 + 5/4 − 3z/4 at height z, and the two triangles 1 out of the solid, to the planes y = −1 and
x = 1. Those two planes cross along the vertical line x = 1, y = −1, and beyond it each is trimmed
by the other: the notch left by the missing quarter is narrowed to the quadrant x ≥ 1, y ≤ −1,
with a sharp reentrant edge on that line. The edges where the triangles meet the side and the base
become pieces of tubes of radius 1 about them, and the apex a piece of a ball of radius 1 about
it.

In the missing quarter the reach is 1 about the cut faces and their edges only: the side is
grown only over its own 270°, so the result is not the full grown solid less a quadrant.

The volume and area have no closed form stated here. The case is unsettled until two kernels
agree.

**As a set:**

$$\{\,\rho \le 6 - \tfrac{3z}{4},\ 0 \le z \le 8,\ 0^\circ \le \varphi \le 270^\circ \,\} \oplus B_1$$
