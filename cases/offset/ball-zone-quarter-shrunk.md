+++
title = "A quarter of a ball cut flat, shrunk by 1"
tags = ["offset", "shrink", "sphere", "pole", "planar"]
origin = "OCCT tests/offset/shape_type_i/B1, B5"
settled = false
+++

A quarter of a ball cut flat: the part of the ball of radius 5 centered on the origin with z ≤ 3
whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 90°, so the part
with x ≥ 0 and y ≥ 0. It runs from the south pole, (0, 0, −5), up to the plane z = 3. Its faces
are the sphere, the flat top (a quarter of the disk of radius 4 at z = 3), and two flat faces, in
the half-planes y = 0, x ≥ 0 and x = 0, y ≥ 0, which meet along the z axis. Shrink it by 1: the
solid of every point at least 1 from its outside.

Each face moves 1 into the solid: the sphere to the sphere of radius 4, the top down to z = 2, and
the two flats to the planes y = 1 and x = 1, which meet in a vertical edge on the line
x = y = 1, running from z = −√14 ≈ −3.74, where the sphere of radius 4 cuts that line, up to
z = 2. The south pole is gone. The sphere meets each moved flat along an arc of a circle of radius
√15 and the moved top along an arc of the circle of radius √12.

The volume and area are integrals of elementary functions over this convex solid. They are left
unsettled until two kernels agree.

**As a set:**

$$\{\,\lVert p \rVert \le 5,\ z \le 3,\ 0^\circ \le \varphi \le 90^\circ \,\} \ominus B_1
= \{\,\lVert p \rVert \le 4,\ z \le 2,\ x \ge 1,\ y \ge 1 \,\}$$
