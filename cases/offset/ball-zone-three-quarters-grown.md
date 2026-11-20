+++
title = "Three quarters of a ball cut flat, grown by 1"
tags = ["offset", "grow", "sphere", "pole", "planar", "reentrant"]
origin = "OCCT tests/offset/shape_type_i/B2, B6"
settled = false
+++

Three quarters of a ball cut flat: the part of the ball of radius 5 centered on the origin with
z ≤ 3 whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 270°,
so everything but the quarter with x ≥ 0 and y ≤ 0. It runs from the south pole, (0, 0, −5), up to
the plane z = 3. Its faces are the sphere, the flat top (three quarters of the disk of radius 4 at
z = 3), and two flat faces, half-disks cut from the half-planes y = 0, x ≥ 0 and x = 0, y ≤ 0,
which meet along the z axis in a reentrant edge, the solid on a three-quarter side of it. Grow it by
1: the solid of every point within 1 of it.

The sphere moves out to the sphere of radius 6, the top up to z = 4, and the two flats 1 out of the
solid, to the planes y = −1 and x = 1. Those two planes cross along the vertical line x = 1,
y = −1, and beyond it each is trimmed by the other: the notch left by the missing quarter is
narrowed to the quadrant x ≥ 1, y ≤ −1, and its inside edge on that line is a sharp reentrant
edge. The edges where the flats meet the sphere and the top become pieces of tubes of radius 1
about them, as for a convex solid.

In the missing quarter the reach is 1 about the cut faces and their edges only: the side is
grown only over its own 270°, so the result is not the full grown solid less a quadrant.

The volume and area have no closed form stated here. The case is unsettled until two kernels
agree.

**As a set:**

$$\{\,\lVert p \rVert \le 5,\ z \le 3,\ 0^\circ \le \varphi \le 270^\circ \,\} \oplus B_1$$
