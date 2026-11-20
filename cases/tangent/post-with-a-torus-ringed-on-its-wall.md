+++
title = "A post united with a torus ringed round it, its core circle on the post's wall"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "torus"]
origin = "OCCT tests/boolean/bopfuse_simple/ZL2"

[volume]
value = 505.08746621045185
exact = "πR²h + π²r²R + 4πr³/3 with R = 4, h = 8, r = 3/2: the post and the half of the torus outside it (Pappus: the half-disc of the tube, area πr²/2, its centroid 4r/3π beyond the core circle)"
tex = '\pi R^2 h + \pi^2 r^2 R + \tfrac{4\pi r^3}{3},\quad R = 4,\ h = 8,\ r = \tfrac32'

[area]
value = 372.90425775384557
exact = "2πR² + 2πRh − 4πRr + 2π²Rr + 4πr²: the post's ends and wall, less the band of wall 2r high the tube covers, plus the outer half of the torus's surface (Pappus on the outer semicircle, arc πr with its centroid 2r/π beyond the core circle)"
tex = '2\pi R^2 + 2\pi R h - 4\pi R r + 2\pi^2 R r + 4\pi r^2'
+++

A post: a cylinder of radius 4, its axis the z axis, from z = 0 to z = 8.

A torus about the same axis, centred at (0, 0, 4), the post's mid-height: the circle of radius 4 in the plane z = 4 about the z axis is its core circle, and its tube has radius 3/2, so the torus is every point within 3/2 of that circle. Make the union of the post and the torus.

The core circle lies on the post's wall, so the post's wall cuts the tube in half along its length: the inner half lies inside the post, the outer half stands out of the wall as a rounded ring, 3/2 proud of it. The tube stays between z = 2.5 and z = 5.5, clear of the post's ends.

**As a set:**

$$\{\,\rho \le 4,\ 0 \le z \le 8\,\} \cup \{\,(\rho - 4)^2 + (z - 4)^2 \le (3/2)^2\,\}$$
