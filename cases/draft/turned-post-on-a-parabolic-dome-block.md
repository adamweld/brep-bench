+++
title = "A turned post on a block under a parabolic dome, its outer side drafted"
tags = ["draft", "cone", "bezier", "revolve", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/G8"
+++

The block: the solid turned about the z axis from the region under the quadratic Bézier arc from
(ρ, z) = (0, 10) through (10, 10) to (20, 4), above z = 0 and out to ρ = 20. The arc is the
parabola z = 10 − 0.015ρ², so the block is a flat-floored disk of radius 20 under a dome, 10 high at
the axis and 4 high at its rim.

A post stands off its axis, turned about the vertical line through (8, 0): the region between 2 and
5 out from that line, 0 ≤ z ≤ 28, of the half plane y = 0, x > 8, turned 180° counterclockwise
(seen from +z) about the line from +x. It is a half tube with an outer face on the cylinder of
radius 5 about the line, an inner face on the cylinder of radius 2, a flat top at z = 28 and two
flat ends in the plane y = 0. It is united with the block; it shows only above the dome.

Draft the post's outer face by 4°, with the pull direction +z and the neutral plane z = 0, the
block's floor: the face turns about the half circle of radius 5 where it crosses the neutral plane,
until it leans 4° from the z direction, inward as it rises, into the cone of radius 5 − z·tan 4°
about the vertical line. The inner face, the two ends, the top and the block are not drafted; the
top is cut back to meet the cone. It has no closed form that has been checked.

**As a set:**

$$\{\, 0 \le z \le 10 - 0.015\rho^2,\ \rho \le 20 \,\} \cup \{\, 2 \le \sqrt{(x-8)^2 + y^2} \le 5 - z\tan 4^\circ,\ 0 \le z \le 28,\ y \ge 0 \,\}$$
