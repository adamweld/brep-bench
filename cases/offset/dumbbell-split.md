+++
title = "A dumbbell shrunk until it splits into two bodies"
tags = ["offset", "shrink", "planar", "self-intersection", "round", "lumps"]

[volume]
value = 295.4404727084366
exact = "2·3(49 + β), β = 3 − √5/2 − (9/4)·asin(2/3); two sections of area 49 + β, each 3 high"
tex = '6(49+\beta),\quad \beta=3-\tfrac{\sqrt5}{2}-\tfrac94\arcsin\tfrac23'

[area]
value = 366.09541295104316
exact = "4(49 + β) + 6(26 + 3·asin(2/3)), β = 3 − √5/2 − (9/4)·asin(2/3)"
tex = '4(49+\beta)+6\left(26+3\arcsin\tfrac23\right),\quad \beta=3-\tfrac{\sqrt5}{2}-\tfrac94\arcsin\tfrac23'
+++

A dumbbell-shaped plate: its outline in the xy plane is the polygon through (0, 0), (10, 0),
(10, 4), (14, 4), (14, 0), (24, 0), (24, 10), (14, 10), (14, 6), (10, 6), (10, 10) and (0, 10),
two 10 × 10 squares joined by a neck 4 long and 2 wide, extruded 6 along +z.

Shrink the whole solid inward by 1.5: keep every point inside it at least 1.5 from its boundary.
The neck disappears, leaving two separate bodies; keep both. The moved walls that cross in the
neck are not part of either body. This is an offset of the solid, not its shell or hollowed wall.

Each body runs from z = 1.5 to z = 4.5. The left one's section is the square from (1.5, 1.5)
to (8.5, 8.5), with a small bulge toward the lost neck. At its mouth the nearest original walls
end at (10, 4) and (10, 6), so the bulge is bounded by arcs of radius 1.5 about those corners.
The arcs leave x = 8.5 tangentially at y = 4 and y = 6 and meet at (10 − √5/2, 5).
The right body is the reflection of the left in x = 12.

The bulge's area is β = 3 − √5/2 − (9/4)·asin(2/3), and each section's perimeter is
26 + 3·asin(2/3), with the angles in radians. Multiply the two section areas by height 3 for
the volume; the four ends and the two perimeters times 3 give the surface area.
