+++
title = "Two equal frusta, one slid sideways, their common part"
tags = ["boolean", "common", "cone"]
origin = "OCCT tests/boolean/bopcommon_simple/ZN2"
settled = false
+++

A cone frustum on the z axis from z = 0 to z = 10, radius 6 at its base and 4 at its top (the radius is 6 − z/5), and a second, equal frustum with its axis the vertical line through (2, 0): the first slid 2 along the x axis. Take the part common to the two.

At every height the two sections are equal discs whose centres are 2 apart, so they overlap in a lens, and the common part is the stack of these lenses from z = 0 to z = 10: a solid with two flat lens-shaped ends and two faces of cone, one from each frustum, meeting along two edges that lie in the plane x = 1 (the two equal cones, one the other slid along x, meet in a plane curve).

**As a set:**

$$\{\rho \le 6 - \tfrac{z}{5},\ 0 \le z \le 10\} \cap \{(x-2)^2 + y^2 \le (6 - \tfrac{z}{5})^2,\ 0 \le z \le 10\}$$
