+++
title = "A turned post on a sector of a frustum, its outer side drafted"
tags = ["draft", "cone", "revolve", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/E9"
+++

The base: a quarter of a frustum of a cone about the z axis, the points with x ≥ 0 and y ≥ 0 whose
distance ρ from the z axis is at most 20 − 0.4z, for z from 0 to 20: radius 20 at the floor z = 0 and
12 at the flat top z = 20, with two flat faces, in the planes x = 0 and y = 0.

A post stands on it, turned about the vertical line through (10, 6): the region between 2 and 5
out from that line, 0 ≤ z ≤ 26, of the half plane y = 6, x > 10, turned 150° counterclockwise
(seen from +z) about the line from +x. It is a curved block with an outer face on the cylinder of
radius 5 about the line, an inner face on the cylinder of radius 2, a flat top at z = 26 and two
flat ends in the vertical planes through the line at 0° and 150°. Its floor lies within the
frustum's floor, and it is united with the base; it rises 6 above the frustum's top and its outer
face crosses the frustum's slanting side.

Draft the post's outer face by 4°, with the pull direction +z and the neutral plane z = 0, the
base's floor: the face turns about the arc of radius 5 where it crosses the neutral plane, until it
leans 4° from the z direction, inward as it rises, into the cone of radius 5 − z·tan 4° about the
vertical line. The inner face, the two ends, the top and the base are not drafted; the top is cut
back to meet the cone. It has no closed form that has been checked.

**As a set:**

$$\{\, x, y \ge 0,\ \rho \le 20 - 0.4z,\ 0 \le z \le 20 \,\} \cup \{\, 2 \le \sqrt{(x-10)^2 + (y-6)^2} \le 5 - z\tan 4^\circ,\ 0 \le z \le 26,\ 0^\circ \le \varphi \le 150^\circ \,\}$$

where φ is the angle about the vertical line through (10, 6), from +x counterclockwise.
