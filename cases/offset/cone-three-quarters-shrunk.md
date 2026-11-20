+++
title = "Three quarters of a cone shrunk by 1"
tags = ["offset", "shrink", "cone", "apex", "planar", "reentrant"]
origin = "OCCT tests/offset/shape_type_i/A8"
settled = false
+++

Three quarters of a cone: the cone on the z axis with a base of radius 6 at z = 0 and its apex at
(0, 0, 8), the part whose angle about the z axis, φ, measured from +x toward +y, lies between 0°
and 270°, so everything but the quarter with x ≥ 0 and y ≤ 0. Its faces are the cone's side, its
base (three quarters of the disk of radius 6), and two triangles with corners at the origin, the
apex and the base's rim, in the half-planes y = 0, x ≥ 0 and x = 0, y ≤ 0, which meet along the z
axis in a reentrant edge. Shrink it by 1: the solid of every point at least 1 from its outside.

The base moves up to z = 1 and the side in along its normal by 1, to the cone of the same angle
with radius 19/4 − 3z/4 at height z. The triangles move 1 into the solid, to the planes y = 1 and
x = −1, which do not meet: the part of space within 1 of the missing quarter is the quadrant
x ≥ −1, y ≤ 1 with its corner at (−1, 1) rounded, by an arc of radius 1 centred on the origin, and the solid's reentrant edge becomes a
concave quarter of a cylinder of radius 1 about the z axis, joining the two moved triangles. The
moved cone's apex, on the z axis at z = 19/3, lies inside that rounding, so the result has no
apex.

The volume and area have no closed form stated here. The case is unsettled until two kernels
agree.

**As a set:**

$$\{\,\rho \le 6 - \tfrac{3z}{4},\ 0 \le z \le 8,\ 0^\circ \le \varphi \le 270^\circ \,\} \ominus B_1$$
