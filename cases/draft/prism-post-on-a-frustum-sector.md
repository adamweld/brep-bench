+++
title = "A triangular post on a sector of a frustum, its slanted side drafted"
tags = ["draft", "planar", "cone", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/E7, OCCT tests/draft/angle/E8"
+++

The base: a quarter of a frustum of a cone about the z axis, the points with x ≥ 0 and y ≥ 0 whose
distance ρ from the z axis is at most 20 − 0.4z, for z from 0 to 20: radius 20 at the floor z = 0 and
12 at the flat top z = 20, with two flat faces, in the planes x = 0 and y = 0.

A post stands on it: the prism over the triangle with corners (8, 4), (14, 4) and (8, 10) in the
plane z = 0, from z = 0 to z = 26, united with the base. Its floor lies within the frustum's floor;
one corner, (14, 4), is beyond the frustum's slanting side at the top rim, and the post rises 6
above the frustum's top.

Draft the post's slanted face, the one through (14, 4) and (8, 10), by 5°, with the pull direction
+z and the neutral plane z = 0, the base's floor: the face turns about its line in that plane until
it leans 5° from the z direction, toward the post's inside as it rises. The post's other two sides,
its top and the base are not drafted; the top and the two sides next to the drafted face are cut
back to meet it. It has no closed form that has been checked.

**As a set:**

$$\{\, x, y \ge 0,\ \rho \le 20 - 0.4z,\ 0 \le z \le 20 \,\} \cup \{\, x \ge 8,\ y \ge 4,\ x + y \le 18 - \sqrt{2}\,z\tan 5^\circ,\ 0 \le z \le 26 \,\}$$
