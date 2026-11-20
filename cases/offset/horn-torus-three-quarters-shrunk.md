+++
title = "Three quarters of a horn torus, upper half, shrunk by 1"
tags = ["offset", "shrink", "torus", "horn", "pinch", "concave"]
origin = "OCCT tests/offset/shape_type_i/G1"
settled = false
+++

A horn torus: the solid swept by the disk of radius 5 centered on (5, 0, 0) in the plane y = 0,
turned about the z axis. Its tube touches the axis at the origin, where its surface pinches to a
point. Take its upper half, z ≥ 0, turned through three quarters of a turn, φ from 0° to 270° (φ
measured from +x toward +y): all but the quarter with x ≥ 0 and y ≤ 0. Its faces are the torus, a
flat disk of radius 10 less its missing quarter in the plane z = 0, and two flat half disks of
radius 5, in the half-plane y = 0, x ≥ 0 and in the half-plane x = 0, y ≤ 0, which touch each other
only at the pinch.

Shrink it by 1: the solid of every point at least 1 from the solid's outside.

The torus moves in to the ring torus whose tube has radius 4 about the circle of radius 5, the flat
moves up to z = 1, and the cut faces move 1 into the solid, to the plane y = 1 (for x ≥ 0) and the
plane x = −1 (for y ≤ 0). The ring torus keeps away from the axis (ρ ≥ 5 − √15 ≈ 1.13), so the round
that shrinking would make at the concave edge, a quarter of a cylinder of radius 1 about the z axis,
lies outside it and no part of it remains.

**As a set:**

$$\{\, (\rho - 5)^2 + z^2 \le 4^2,\ z \ge 1,\ d\bigl((x, y),\ \{x \ge 0,\ y \le 0\}\bigr) \ge 1 \,\}$$

Its volume and area are not worked out in closed form here, so the case is unsettled until two kernels agree.
