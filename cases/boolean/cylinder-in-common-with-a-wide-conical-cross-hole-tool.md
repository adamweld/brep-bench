+++
title = "A cylinder and a frustum across it that is wider than the cylinder is tall, their common part"
tags = ["boolean", "common", "cone", "cylinder"]
origin = "OCCT tests/boolean/bopcommon_simple/ZK7"
settled = false
+++

A cylinder of radius 5 on the z axis, from z = 0 to z = 10, and a cone frustum lying across it: its axis is the line parallel to the x axis at height z = 5 through the z axis, from x = −7 to x = 7, of radius 2 at x = −7 and radius 8 at x = 7 (the radius is 5 + 3x/7). Take the part common to the two.

The frustum crosses the cylinder's axis at a right angle at mid-height and is longer than the cylinder is wide. Its radius passes 5 at x = 0, so on the side x > 0 it is wider than the cylinder is tall and overshoots the cylinder's top and bottom faces; on the other side it lies between them. The common part is the stretch of the frustum inside the cylinder: bounded by the cylinder's wall at each end and, where the frustum overshoots, by the flat top and bottom faces.

**As a set:**

$$\{\rho \le 5,\ 0 \le z \le 10\} \cap \{y^2 + (z-5)^2 \le (5 + \tfrac{3x}{7})^2,\ -7 \le x \le 7\}$$
