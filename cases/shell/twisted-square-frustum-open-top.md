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

Shell it inward to a wall 0.5 thick, open at its top face, z = 8. The side's moved faces run on
past the plane of the opening and are cut by it: the sides are taken as continued beyond z = 8,
and the rim is where their offset meets the plane z = 8. Exactly: let E be the solid continued
up to z = 10 by the same loft, each corner's line run on past z = 8 to z = 10 and the squares
between them closing it. The cavity is E shrunk by 0.5, every point of E at least 0.5 from E's
boundary (its sides, its floor and its far end), cut off at the plane z = 8; the far end is more
than 0.5 above it. The result is the solid less that cavity.

The inward offset of a saddle patch has no closed form, so no volume or area is stated.

**As a set:**

$$S \setminus \bigl((E \ominus B_{1/2}) \cap \{z \le 8\}\bigr), \quad E \text{ the solid continued to } z = 10,\ S = E \cap \{z \le 8\}$$
