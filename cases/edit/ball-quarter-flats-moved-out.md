+++
title = "A quarter of a ball, both its flats moved out past its axis"
tags = ["edit", "move-face", "sphere", "pole", "planar", "analytic"]
origin = "OCCT tests/offset/faces_type_i/C7"

[volume]
value = 2539.045818763064
exact = "4πR³/3 − 2π(R − d)²(2R + d)/3 + K, K = (4R³/3)·atan(e/R) − (2d/3)(3R² − d²)·atan(e/d) + 2d²e/3, R = 10, d = 4, e = √(R² − 2d²) = √68: the ball less the two caps beyond x = −d and y = −d, plus K, the part of the ball beyond both, which the two caps share (∫ over z of each slice's corner x, y ≥ d)"
tex = '\tfrac{4}{3}\pi R^3 - \tfrac{2\pi}{3}(R-d)^2(2R+d) + K,\quad K = \tfrac{4R^3}{3}\arctan\tfrac{e}{R} - \tfrac{2d}{3}\left(3R^2 - d^2\right)\arctan\tfrac{e}{d} + \tfrac{2}{3}d^2 e,\quad R = 10,\ d = 4,\ e = \sqrt{R^2 - 2d^2}'

[area]
value = 1005.1564802365912
exact = "4πRd + 4R(R·atan(e/R) − d·atan(e/d)) + 2(πc² − c²·acos(d/c) + de), c = √(R² − d²) = √84: the sphere less the two zones beyond the flats plus the part of it beyond both, and the two flats, each the disk of radius c less the segment beyond the other flat"
tex = '4\pi R d + 4R\left(R\arctan\tfrac{e}{R} - d\arctan\tfrac{e}{d}\right) + 2\left(\pi c^2 - c^2\arccos\tfrac{d}{c} + d e\right),\quad c = \sqrt{R^2 - d^2}'
+++

A quarter of the ball of radius 10 centered on the origin, from pole to pole: the part whose angle
about the z axis, φ, measured from +x toward +y, lies between 0° and 90°, so the part with x ≥ 0
and y ≥ 0. Its poles are on the z axis, at (0, 0, ±10). Its faces are the sphere and two flat half
disks, in the half-plane y = 0, x ≥ 0 and in the half-plane x = 0, y ≥ 0, which meet along the z
axis in a convex edge from pole to pole.

Then edit the solid directly: move each flat 4 outward along its normal, the first to the plane
y = −4 and the second to the plane x = −4. The faces beside them follow: the sphere runs on round
past the old edge to meet both moved flats, and the two flats are extended until they meet each
other, along the line x = y = −4 from z = −√68 to z = √68. The poles, where the old edge met the
sphere, are left inside the spherical face.

So the result is the ball less everything at x < −4 or y < −4. Each flat is the disk of radius √84
in which its plane cuts the ball, less the segment beyond the other flat's plane.

**As a set:**

$$B_{10} \cap \{\, x \ge -4,\ y \ge -4 \,\}$$
