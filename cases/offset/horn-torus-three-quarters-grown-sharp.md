+++
title = "Three quarters of a horn torus, upper half, grown by 1, its edges kept sharp"
tags = ["offset", "grow", "sharp-edge", "torus", "horn", "pinch", "concave"]
origin = "OCCT tests/offset/shape_type_i/F9"
settled = false
+++

A horn torus: the solid swept by the disk of radius 5 centered on (5, 0, 0) in the plane y = 0,
turned about the z axis. Its tube touches the axis at the origin, where its surface pinches to a
point. Take its upper half, z ≥ 0, turned through three quarters of a turn, φ from 0° to 270° (φ
measured from +x toward +y): all but the quarter with x ≥ 0 and y ≤ 0. Its faces are the torus, a
flat disk of radius 10 less its missing quarter in the plane z = 0, and two flat half disks of
radius 5, in the half-plane y = 0, x ≥ 0 and in the half-plane x = 0, y ≤ 0, which touch each other
only at the pinch.

Grow it by 1 with its edges kept sharp: move each face 1 outward along its normal, and extend the
moved faces until they meet, so that no round is added and every edge stays where its two faces
meet.

The torus moves out to the surface of every point 1 outside it: the tube of radius 6 about the same
circle, which crosses itself on the axis, so the pinch is swallowed. The flat moves down to z = −1.
The cut faces move out 1, the first to the plane y = −1 (for x ≥ 1) and the second to the plane x =
1 (for y ≤ −1), and are extended until they meet each other along the line x = 1, y = −1, so the
missing quarter shrinks to the part with x ≥ 1 and y ≤ −1.

**As a set:**

$$\{\, (\rho - 5)^2 + z^2 \le 6^2,\ z \ge -1 \,\} \setminus \{\, x > 1,\ y < -1 \,\}$$

Its volume and area are not worked out in closed form here, so the case is unsettled until two kernels agree.
