+++
title = "A widening cone post on a sector of a frustum, its side drafted the other way"
tags = ["draft", "cone", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/E6"
+++

The base: a quarter of a frustum of a cone about the z axis, the points with x ≥ 0 and y ≥ 0 whose
distance ρ from the z axis is at most 20 − 0.4z, for z from 0 to 20: radius 20 at the floor z = 0 and
12 at the flat top z = 20, with two flat faces, in the planes x = 0 and y = 0.

A post stands on it: the cone about the vertical line through (10, 6), from z = 0 to z = 26, its
radius about that line 3 at z = 0 and growing as it rises, 3 + z·tan 2°. Its floor lies within the
frustum's floor, and it is united with the base. Near the top the post lies partly beyond the
frustum's slanting side and partly on its top face, and it rises 6 above the frustum's top.

Draft the post's side by 5°, with the pull direction +z and the neutral plane z = 0, the base's
floor: the side turns about the circle of radius 3 about (10, 6) where it crosses the neutral
plane, until it leans 5° from the z direction, inward as it rises. It becomes the cone through that
circle with radius 3 − z·tan 5° about the vertical line, about 1.73 at z = 26. The solid to build is this result: the post's wall narrows upward, radius 3 − z·tan 5°; the widening cone above is only a starting point. The base and the
post's top are not drafted; the top is cut back to meet the cone. It has no closed form that has been checked.

**As a set:**

$$\{\, x, y \ge 0,\ \rho \le 20 - 0.4z,\ 0 \le z \le 20 \,\} \cup \{\, \sqrt{(x - 10)^2 + (y - 6)^2} \le 3 - z\tan 5^\circ,\ 0 \le z \le 26 \,\}$$
