+++
title = "An elliptic cone frustum shrunk by 1"
tags = ["offset", "shrink", "ellipse", "cone", "loft"]
origin = "OCCT tests/bugs/modalg_2/bug5805_33"
settled = false
+++

An elliptic cone frustum on the z axis: the ellipse x²/36 + y²/16 ≤ 1 in the plane z = 0 and the
ellipse x²/9 + y²/4 ≤ 1 in the plane z = 8 (the same shape at half the size), joined by the ruled
surface of straight lines between points of equal parameter, (6 cos u, 4 sin u, 0) to
(3 cos u, 2 sin u, 8): the side of the elliptic cone through them. It is closed by the two
ellipses' flat insides.

Shrink it by 1: the solid of every point at least 1 from its outside. The ends move in to z = 1
and z = 7; the side moves in along its normals by 1, onto a surface that is no cone (the ellipses' curvature radii, at least 16/6 and 4/3, stay
above 1). The edges stay sharp. The solid has no
simple closed form: the offset of an elliptic cone is not a quadric, and its volume and
area are left unsettled.

The result is the part of the frustum at least 1 from its outside, between the planes z = 1 and
z = 7 and inside the surface 1 in from the side, so that it has three faces: that offset surface
and the two flat ends, each trimmed by the other.

**As a set:**

$$\{\,(x, y, z) \text{ in the frustum}\,\} \ominus B_{1}$$
