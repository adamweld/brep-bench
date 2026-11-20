+++
title = "A quarter of the upper half of a horn torus, both its cut flats moved out past the pinch"
tags = ["edit", "move-face", "torus", "pole", "planar"]
origin = "OCCT tests/offset/faces_type_i/K2"
settled = false
+++

A horn torus: the solid swept by the disk of radius 5 centered on (5, 0, 0) in the plane y = 0,
turned about the z axis. Its tube touches the axis at the origin, where its surface pinches to a
point. Take its upper half, z ≥ 0, and of that the quarter turn from φ = 0° to φ = 90°, where φ is
the angle about the z axis from +x toward +y. Its faces are the torus; a flat quarter disk of radius
10 in the plane z = 0, whose corner is the pinch; and two flat half disks of radius 5 above z = 0,
one in the plane y = 0 centered on (5, 0, 0), whose outward normal is −y, and one in the plane
x = 0 centered on (0, 5, 0), whose outward normal is −x. They touch at the pinch.

Then edit the solid directly: move each of the two half disks 3 outward along its outward normal,
to the planes y = −3 and x = −3. The torus runs on round the axis to meet them, and the flat in
z = 0 runs on with it, so the two moved flats meet along the line x = y = −3, in an edge that is
there only for the heights where the tube reaches that line. The pinch, at the origin, is left a
point where the torus touches the middle of the flat in z = 0.

So the result is the upper half of the horn torus less its parts with x < −3 or y < −3. It has no
closed form that is cheap to derive: its volume is ∫ over ρ of ρ·h(ρ)·L(ρ), with h(ρ) =
√(10ρ − ρ²) the height of the tube at distance ρ from the axis and L the angle about the axis at
distance ρ where both x ≥ −3 and y ≥ −3, which is 2π for ρ ≤ 3, 2π − 4·acos(3/ρ) up to ρ = 3√2
and π/2 + 2·asin(3/ρ) beyond.

**As a set:**

$$\{\, (\rho - 5)^2 + z^2 \le 25,\ z \ge 0,\ x \ge -3,\ y \ge -3 \,\}$$
