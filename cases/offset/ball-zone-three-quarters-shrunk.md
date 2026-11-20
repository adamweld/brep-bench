+++
title = "Three quarters of a ball cut flat, shrunk by 1"
tags = ["offset", "shrink", "sphere", "pole", "planar", "reentrant"]
origin = "OCCT tests/offset/shape_type_i/B3, B7"
settled = false
+++

Three quarters of a ball cut flat: the part of the ball of radius 5 centered on the origin with
z ≤ 3 whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 270°,
so everything but the quarter with x ≥ 0 and y ≤ 0. It runs from the south pole, (0, 0, −5), up to
the plane z = 3. Its faces are the sphere, the flat top (three quarters of the disk of radius 4 at
z = 3), and two flat faces, half-disks cut from the half-planes y = 0, x ≥ 0 and x = 0, y ≤ 0,
which meet along the z axis in a reentrant edge. Shrink it by 1: the solid of every point at least
1 from its outside.

The sphere moves in to the sphere of radius 4 and the top down to z = 2. The flats move 1 into the
solid, to the planes y = 1 and x = −1. Those planes do not meet: the missing quarter's neighbourhood
within 1 of it is the quadrant x ≥ −1, y ≤ 1 with its corner at (−1, 1) rounded, by an arc of radius 1 centred on the origin, and the
solid's reentrant edge becomes a concave quarter of a cylinder of radius 1 about the z axis, joining
the two moved flats, with the sphere cutting it at the bottom and the top at z = 2. The south pole
is gone.

The volume and area have no closed form stated here. The case is unsettled until two kernels
agree.

**As a set:**

$$\{\,\lVert p \rVert \le 5,\ z \le 3,\ 0^\circ \le \varphi \le 270^\circ \,\} \ominus B_1$$
