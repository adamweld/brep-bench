+++
title = "A tall cylinder and a frustum lying across it, their common part"
tags = ["boolean", "common", "cone", "cylinder"]
origin = "OCCT tests/boolean/bopcommon_simple/ZK5"
settled = false
+++

A cylinder of radius 5 on the z axis, from z = 0 to z = 20, and a cone frustum lying across it: its axis is the line parallel to the x axis at height z = 10 through the z axis, from x = −7 to x = 7, of radius 1 at x = −7 and radius 3 at x = 7 (the radius is 2 + x/7). Take the part common to the two.

The frustum crosses the cylinder's axis at a right angle at mid-height, is longer than the cylinder is wide and narrower than it is tall, so it passes right through and is cut at both ends by the cylinder's wall; the cylinder's ends touch nothing. The common part is the stretch of the frustum inside the cylinder, closed at each end by a patch of the cylinder's wall.

**As a set:**

$$\{\rho \le 5,\ 0 \le z \le 20\} \cap \{y^2 + (z-10)^2 \le (2 + \tfrac{x}{7})^2,\ -7 \le x \le 7\}$$
