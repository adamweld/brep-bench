+++
title = "A cylinder united with a frustum lying across it, with one end wider than the cylinder"
tags = ["boolean", "union", "cylinder", "cone", "crossing"]
origin = "OCCT tests/boolean/bopfuse_simple/ZK7"
settled = false
+++

A cylinder of radius 5, its axis the z axis, from z = 0 to z = 10.

A frustum of a cone lying across it, its axis the line through (0, 0, 5) parallel to the x axis, crossing the cylinder's axis at right angles at its mid-height: from x = −8 to x = 8, radius 7 at x = −8, narrowing in a straight line to radius 2 at x = 8, so its radius at x is 9/2 − 5x/16. Make the union of the two.

Where the frustum's radius is above 5 (for x below −1.6) its surface runs out of the cylinder's top and bottom faces as well as its wall. Its wide end, at x = −8, lies wholly outside the cylinder, as does its narrow end at x = 8.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup \{\,-8 \le x \le 8,\ y^2 + (z - 5)^2 \le (\tfrac92 - \tfrac{5x}{16})^2\,\}$$
