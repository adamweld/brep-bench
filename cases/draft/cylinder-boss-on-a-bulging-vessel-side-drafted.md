+++
title = "A cylindrical boss on a bulging vessel, its side drafted into a cone"
tags = ["draft", "boolean", "bezier", "revolve", "cylinder", "cone", "unsettled"]
origin = "OCCT tests/draft/angle/I9"
settled = false
+++

The same vessel as in `draft/box-boss-on-a-bulging-vessel-side-drafted`, with a cylinder in place of
the box, whose side the draft turns into a cone.

The vessel: the region in the xz plane (x ≥ 0) bounded by the segment of the z axis from (0, 0) to
(0, 22), the segment of the x axis from (0, 0) to (10, 0), the cubic Bézier curve with control points
(10, 0), (16, 6), (16, 16), (10, 22), and the segment from (10, 22) back to (0, 22), turned about the
z axis through half a turn, from +x toward +y, so that the vessel lies in y ≥ 0. Its wall bulges from
radius 10 at the bottom to 14.5 at mid-height (z = 11) and back to 10 at the top.

The boss: the cylinder of radius 4 about the vertical line through (13, 6), from z = 4 to z = 18,
united with the vessel. Its axis is 14.3 from the vessel's, so the wall cuts across the cylinder.

Draft the cylinder's side by 4°, with the pull direction +z and the neutral plane z = 4, the
cylinder's bottom: the side becomes a cone through the circle of radius 4 at z = 4, leaning 4° from
the z direction, inward as it rises, its radius 4 − (z − 4)·tan 4°, to about 3.02 at z = 18. It
still runs into the wall, now along another curve. The vessel and the cylinder's bottom are not
drafted; the top shrinks to meet the cone.

It has no closed form: the volume and area involve the Bézier wall's intersection with the cone.

**As a set:**

$$\mathrm{vessel} \ \cup\ \bigl\{\, (x - 13)^2 + (y - 6)^2 \le \bigl(4 - (z - 4)\tan 4^\circ\bigr)^2,\
4 \le z \le 18 \,\bigr\}$$
