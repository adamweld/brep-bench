+++
title = "A cube and a post along one of its edges, their common part"
tags = ["tangent", "boolean", "common", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcommon_simple/Z9"

[volume]
value = 196.349540849362077
exact = "V = 10·(25π/4) = 125π/2: a quarter of the post's disc, 10 high"
tex = '\tfrac{125\pi}{2}'

[area]
value = 217.809724509617246
exact = "area = 2·(25π/4) + 10·(5π/2) + 2·(5·10) = 75π/2 + 100: the quarter disc top and bottom, the wall's quarter, and the two flat faces 5 by 10"
tex = '\tfrac{75\pi}{2} + 100'
+++

A cube with edge 10: x, y and z each from 0 to 10.

A post: a solid cylinder of radius 5, its axis the vertical line through the cube's edge at x = 0, y = 0, from z = −5 to z = 10. Its top disc is level with the cube's top face and it extends 5 below the cube's base. Take the part common to the cube and the post.

The post's axis lies along the cube's vertical edge, so a quarter of the post's disc lies inside the cube, to a height of 10. What is common is a quarter-round: a quarter of a cylinder of radius 5 and height 10, its two flat faces on the cube's faces x = 0 and y = 0.

**As a set:**

$$\bigl([0,10]^3\bigr) \cap \{\,\rho \le 5,\ -5 \le z \le 10\,\}$$
