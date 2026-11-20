+++
title = "A tall cylinder and a frustum lying across it that ends inside it, their common part"
tags = ["boolean", "common", "cone", "cylinder"]
origin = "OCCT tests/boolean/bopcommon_simple/ZK9"
settled = false
+++

A cylinder of radius 5 on the z axis, from z = 0 to z = 20, and a cone frustum lying across it: its axis is the line parallel to the x axis at height z = 10 through the z axis, from x = −7 to x = 1, of radius 1 at x = −7 and radius 4 at x = 1 (the radius is 1 + 3(x + 7)/8). Take the part common to the two.

The frustum's narrow end sticks out of the cylinder on the side x < 0, where the cylinder's wall cuts it; its wide end, a flat disc of radius 4 in the plane x = 1, lies wholly inside the cylinder. The common part is the stretch of the frustum from the cylinder's wall to that disc.

**As a set:**

$$\{\rho \le 5,\ 0 \le z \le 20\} \cap \{y^2 + (z-10)^2 \le (1 + \tfrac{3(x+7)}{8})^2,\ -7 \le x \le 1\}$$
