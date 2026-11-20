+++
title = "Three quarters of the upper half of a horn torus, one cut flat moved in"
tags = ["edit", "move-face", "torus", "pole", "planar", "concave"]
origin = "OCCT tests/offset/faces_type_i/K6"
settled = false
+++

A horn torus: the solid swept by the disk of radius 5 centered on (5, 0, 0) in the plane y = 0,
turned about the z axis. Its tube touches the axis at the origin, where its surface pinches to a
point. Take its upper half, z ≥ 0, and of that the three quarters of a turn from φ = 0° to
φ = 270°, where φ is the angle about the z axis from +x toward +y. Its faces are the torus; a flat
three-quarter disk of radius 10 in the plane z = 0, whose corner is the pinch; and two flat half
disks of radius 5 above z = 0, one in the plane y = 0 centered on (5, 0, 0), whose outward normal
is −y, and one in the plane x = 0 centered on (0, −5, 0), whose outward normal is +x. They touch at
the pinch, where the solid has a concave corner.

Then edit the solid directly: move the half disk in y = 0 inward 2, against its outward normal, to
the plane y = 2. The faces beside it are trimmed: the torus and the flat in z = 0 lose their parts
with x > 0 and y < 2, and the other half disk, in x = 0, runs on to y ≤ 2 as far as the tube
reaches. The two flats meet along the line x = 0, y = 2.

So the result is the upper half of the horn torus less its part with x > 0 and y < 2. It has no
closed form that is cheap to derive: that part's volume is ∫ over ρ of ρ·h(ρ)·(π/2 + asin(min(1,
2/ρ))), with h(ρ) = √(10ρ − ρ²) the height of the tube at distance ρ from the axis.

**As a set:**

$$\{\, (\rho - 5)^2 + z^2 \le 25,\ z \ge 0 \,\} \setminus \{\, x > 0,\ y < 2 \,\}$$
