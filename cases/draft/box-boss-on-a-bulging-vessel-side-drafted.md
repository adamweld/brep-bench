+++
title = "A box boss on a bulging vessel, its side drafted where it meets the wall"
tags = ["draft", "boolean", "bezier", "revolve", "plane", "unsettled"]
origin = "OCCT tests/draft/angle/I8"
settled = false
+++

A vessel turned half a turn from a plane region with a bulging wall, a box fused to its outside, and
the box's side drafted where it runs into the wall.

The vessel: the region in the xz plane (x ≥ 0) bounded by the segment of the z axis from (0, 0) to
(0, 22), the segment of the x axis from (0, 0) to (10, 0), the cubic Bézier curve with control points
(10, 0), (16, 6), (16, 16), (10, 22), and the segment from (10, 22) back to (0, 22), turned about the
z axis through half a turn, from +x toward +y, so that the vessel lies in y ≥ 0. Its wall is a
surface of revolution from radius 10 at the bottom, to 14.5 at mid-height (z = 11), and back to 10 at
the top; its two flat faces lie in the plane y = 0.

The boss: the box with corners (9, 2, 6) and (20, 8, 16), united with the vessel. Its inner end is
within the vessel, and the rest stands out through the wall, whose bulge it meets across its
side faces.

Draft the box's side face y = 8, the part of it that stands outside the vessel, by 4°, with the pull
direction +z and the neutral plane z = 6, the box's bottom: the face turns about its line at z = 6,
leaning toward the box's inside, −y, as it rises, to y = 8 − 10·tan 4° at the box's top. It still
runs into the wall, now along another curve. The vessel, the box's other faces and its bottom are not
drafted; the top face shrinks to meet the drafted side.

It has no closed form: the volume and area involve the Bézier wall's intersection with the boss.

**As a set:**

$$\mathrm{vessel} \ \cup\ \{\, 9 \le x \le 20,\ 2 \le y \le 8 - (z - 6)\tan 4^\circ,\ 6 \le z \le 16 \,\}$$
