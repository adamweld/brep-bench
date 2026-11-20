+++
title = "A twisted square frustum shrunk by 0.5"
tags = ["offset", "shrink", "loft", "ruled", "bilinear"]
origin = "OCCT tests/bugs/modalg_2/bug5805_37, OCCT tests/bugs/modalg_2/bug5805_38"
settled = false
+++

A solid made by a ruled loft between two squares, closed by the two squares' faces. The upper
square has its center at (0, 0, 8) and sides of 4 along x and y: corners (±2, ±2, 8). The lower
square lies in the plane z = 0, has sides of 8, and its center at (1, 0, 0), turned 30° about
that center, right-handed about +z: its corners are (1, 0, 0) + R·(4 sx, 4 sy, 0) for the four
signs (sx, sy), R the turn by 30°. The loft joins each upper corner (2 sx, 2 sy, 8) to the lower
corner of the same signs, and each upper edge to the lower edge between the corresponding corners,
by straight lines between points at the same fraction along the two edges. Each side face is the
bilinear patch between a lower and an upper edge: a saddle, not a plane.

Shrink the whole solid by 0.5, no face opened: the solid of every point at least 0.5 from its
boundary (its four saddle sides, its top and its bottom).

The inward offset of a saddle patch has no closed form, so no volume or area is stated.

**As a set:**

$$S \ominus B_{1/2}$$
