+++
title = "A plate's three edges at a corner rounded with unequal radii, creased where they meet"
tags = ["fillet", "corner", "cylinder", "planar"]
settled = false
origin = "OCCT blend/simple/A4"
+++

A plate, the box from the origin to (20, 12, 4). Three of its edges meet at the corner (20, 0, 4):
the edge along x where the faces y = 0 and z = 4 meet, rounded to radius 1; the edge along y where the
faces x = 20 and z = 4 meet, rounded to radius 2; and the upright edge along z where x = 20 meets
y = 0, rounded to radius 1.5.

The radii differ, so the rounds cannot close the corner with a patch of one sphere. Each round is a
quarter cylinder running the whole length of its edge, tangent to both of the edge's faces, and none is
shortened by another: where they cross they meet in sharp creases. The solid is the plate less what
each of the three rounds removes.

**As a set:**

$$[0,20]\times[0,12]\times[0,4] \setminus \big( C_x \cup C_y \cup C_z \big)$$

where $C_x = \{\, y \le 1,\ z \ge 3,\ (y-1)^2 + (z-3)^2 \ge 1 \,\}$,
$C_y = \{\, x \ge 18,\ z \ge 2,\ (x-18)^2 + (z-2)^2 \ge 4 \,\}$ and
$C_z = \{\, x \ge 18.5,\ y \le 1.5,\ (x-18.5)^2 + (y-1.5)^2 \ge 1.5^2 \,\}$.
