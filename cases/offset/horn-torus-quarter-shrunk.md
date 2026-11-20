+++
title = "A quarter of a horn torus, upper half, shrunk by 1"
tags = ["offset", "shrink", "torus", "horn", "pinch"]
origin = "OCCT tests/offset/shape_type_i/F8"
settled = false
+++

A horn torus: the solid swept by the disk of radius 5 centered on (5, 0, 0) in the plane y = 0,
turned about the z axis. Its tube touches the axis at the origin, where its surface pinches to a
point. Take its upper half, z ≥ 0, turned only through a quarter of a turn, φ from 0° to 90° (φ
measured from +x toward +y). Its faces are the torus, a flat quarter disk of radius 10 in the plane
z = 0, and two flat half disks of radius 5, in the half-plane y = 0, x ≥ 0 and in the half-plane x =
0, y ≥ 0, which touch each other only at the pinch.

Shrink it by 1: the solid of every point at least 1 from the solid's outside.

The torus moves in to the ring torus whose tube has radius 4 about the circle of radius 5, which no
longer touches the axis; the flat moves up to z = 1, and the cut faces to the planes y = 1 and x =
1. The pinch is cut away.

**As a set:**

$$\{\, (\rho - 5)^2 + z^2 \le 4^2,\ z \ge 1,\ x \ge 1,\ y \ge 1 \,\}$$

Its volume and area are not worked out in closed form here, so the case is unsettled until two kernels agree.
