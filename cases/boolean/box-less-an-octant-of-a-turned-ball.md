+++
title = "A box less the octant of a ball turned off the axes"
tags = ["boolean", "cut", "analytic", "sphere", "planar", "pole", "seam"]
origin = "OCCT boolean/bcut_simple/A3, OCCT boolean/bcut_simple/A5"

[volume]
value = 446.55015305021264
exact = "8³ − (1/8)·4πr³/3 = 512 − 125π/6, r = 5"
tex = '8^3 - \tfrac18\cdot\tfrac43\pi r^3 = 512 - \tfrac{125\pi}{6},\quad r = 5'

[area]
value = 364.36504591506379
exact = "6·8² − 3·πr²/4 + (1/8)·4πr² = 384 − 25π/4"
tex = '6\cdot 8^2 - 3\cdot\tfrac{\pi r^2}{4} + \tfrac18\cdot 4\pi r^2 = 384 - \tfrac{25\pi}{4}'
+++

The box from (0, 0, 0) to (8, 8, 8), and a ball of radius 5 centered on the origin, bounded by a plain spherical surface (not a B-spline): the sphere with its axis along (1, 2, 2)/3 and its seam, the meridian where the surface's angle about the axis wraps, in the half-plane through that axis and the direction (2, 1, −2)/3. So the ball's poles are at ±(5/3, 10/3, 10/3), and neither the poles nor the seam lie in any of the planes x = 0, y = 0 or z = 0. Cut the ball from the box.

The box's corner at the center of the ball is inside it and the box's other seven corners are outside, so the ball takes the octant x, y, z ≥ 0 of itself out of the box's corner, a hollow shaped like an eighth of a ball. The ball does not reach the box's other three faces. Placing the box in another octant of the ball, say from (−8, 0, 0) to (0, 8, 8), changes only where the seam and the poles meet the cut.

**As a set:**

$$[0,8]^3 \setminus B_5$$
