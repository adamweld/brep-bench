+++
title = "A box boss on the arched top of a hollow block, its side drafted"
tags = ["draft", "boolean", "shell", "cylinder", "plane", "unsettled"]
origin = "OCCT tests/draft/angle/I7"
settled = false
+++

A closed hollow block with an arched top, a box fused over the crown of the arch and into its cavity,
and the box's side drafted where it stands out of the arch.

The block: the prism along y, from y = 0 to y = 20, of the plane region x from 0 to 40, bounded below
by z = 0 and above by the arc of the circle of radius 52 about (20, −40) in the xz plane (so z = 8 at
x = 0 and at x = 40, and 12 at x = 20, the crown). Its top is a cylinder of radius 52, its bottom, ends
and sides are flat.

The hollow: the block less its inward offset by 2, the points of the block at least 2 from every face.
For this convex block that is the box-and-arch x from 2 to 38, y from 2 to 18, z from 2 up to the
cylinder of radius 50 about the same axis (the crown at z = 10); the cavity is closed, and the wall 2
thick all round.

The boss: the box with corners (14, 6, 9) and (26, 14, 18), united with the hollowed block. Its
bottom, at z = 9, is below the cavity's roof (z ≈ 9.64 at x = 14, 10 at x = 20), so the box fills a
slab of the cavity under the crown, and it stands out of the arch from there to its top at z = 18.

Draft the part of the box's side face y = 14 that stands out above the arch by 5°, with the pull
direction +z and the neutral plane z = 11, a height within the wall: the face turns about its line at
z = 11, leaning toward the box's inside, −y, as it rises, to y = 14 − 7·tan 5° at the box's top. The
part of that side inside the cavity, below z = 11, is not drafted. The block's faces and the box's other
faces are not drafted; the top shrinks to meet the drafted side.

It has no closed form: the volume and area involve the arch's intersection with the boss.

**As a set:**

$$\mathrm{hollow\ block} \ \cup\ \{\, 14 \le x \le 26,\ 6 \le y \le 14 - \max(z - 11, 0)\tan 5^\circ,\
9 \le z \le 18 \,\}$$
