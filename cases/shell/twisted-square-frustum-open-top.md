+++
title = "A twisted square frustum shelled with its narrow top open"
tags = ["shell", "inward", "loft", "ruled", "bilinear"]
origin = "OCCT tests/bugs/modalg_2/bug5805_38"
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

Shell it inward to a wall 0.5 thick, open at its top face, z = 8. The cavity is every point of
the solid at least 0.5 from its side faces and at least 0.5 above its bottom, running up to the top
plane, which is the opening; the wall is everything else in the solid. The result is the solid
less that cavity.

The inward offset of a saddle patch has no closed form, so no volume or area is stated.

**As a set:**

$$S \setminus \{\, p \in S : d(p,\ \partial S \cap \{z < 8\}) \ge \tfrac12 \,\}, \quad S \text{ the lofted solid}$$
