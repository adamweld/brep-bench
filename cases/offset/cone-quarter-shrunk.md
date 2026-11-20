+++
title = "A quarter of a cone shrunk by 1"
tags = ["offset", "shrink", "cone", "apex", "planar"]
origin = "OCCT tests/offset/shape_type_i/A6"
settled = false
+++

A quarter of a cone: the cone on the z axis with a base of radius 6 at z = 0 and its apex at
(0, 0, 8), the part whose angle about the z axis, φ, measured from +x toward +y, lies between 0°
and 90°, so the part with x ≥ 0 and y ≥ 0. Its faces are the cone's side, its base (a quarter of
the disk of radius 6), and two triangles with corners at the origin, the apex and the base's rim,
in the half-planes y = 0, x ≥ 0 and x = 0, y ≥ 0, which meet along the z axis. Shrink it by 1: the
solid of every point at least 1 from its outside.

The base moves up to z = 1, the two triangles into the solid to the planes y = 1 and x = 1, and
the side in along its normal by 1, to the cone of the same angle with radius 19/4 − 3z/4 at height
z (a step of 1 along the normal is a step of 5/4 across). The apex is gone: the moved side would
reach its axis at z = 19/3, but the moved triangles' edge, the vertical line x = y = 1, meets
that cone at z = (19/4 − √2)·4/3 ≈ 4.45, and the solid ends there, so the result is bounded by the
moved base, the moved side, and the two moved triangles, which meet along the line x = y = 1
from z = 1 up to that point.

The volume and area are integrals of elementary functions over this convex solid. They are left
unsettled until two kernels agree.

**As a set:**

$$\{\,\rho \le 6 - \tfrac{3z}{4},\ 0 \le z \le 8,\ 0^\circ \le \varphi \le 90^\circ \,\} \ominus B_1
= \{\,\rho \le \tfrac{19}{4} - \tfrac{3z}{4},\ z \ge 1,\ x \ge 1,\ y \ge 1 \,\}$$
