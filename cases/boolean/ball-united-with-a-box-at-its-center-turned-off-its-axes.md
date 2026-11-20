+++
title = "A ball united with a box at its center, the ball turned off the axes"
tags = ["boolean", "union", "analytic", "sphere", "planar", "pole", "seam"]
origin = "OCCT boolean/bfuse_simple/A2, OCCT boolean/bfuse_simple/A1, OCCT boolean/bfuse_simple/A4"

[volume]
value = 970.14892864851151
exact = "7/8·4πr³/3 + 8³ = 875π/6 + 512, r = 5"
tex = '\tfrac78\cdot\tfrac43\pi r^3 + 8^3 = \tfrac{875\pi}{6} + 512,\quad r = 5'

[area]
value = 599.98449493429829
exact = "7/8·4πr² + (8³ faces: 3·8² outside, 3·(8² − πr²/4) the faces at the center) = 175π/2 + 384 − 75π/4 = 275π/4 + 384"
tex = '\tfrac78\cdot 4\pi r^2 + 384 - 3\cdot\tfrac{\pi r^2}{4} = \tfrac{275\pi}{4} + 384'
+++

A ball of radius 5 centered on the origin, bounded by a plain spherical surface (not a B-spline): the sphere with its axis along (1, 2, 2)/3 and its seam, the meridian where the surface's angle about the axis wraps, in the half-plane through that axis and the direction (2, 1, −2)/3. So its poles are at ±(5/3, 10/3, 10/3), and neither the poles nor the seam lie in any of the planes x = 0, y = 0 or z = 0. Unite it with the box from (0, 0, 0) to (8, 8, 8).

The box's corner at the center is inside the ball and its other seven corners are outside, so the box shares the octant x, y, z ≥ 0 of the ball and the union is the ball with the rest of the box added: the box's three faces at x = 0, y = 0 and z = 0 are inside the ball's octant for a quarter disk of radius 5 each, and those quarter disks are interior and go; the box's other three faces lie wholly outside the ball. Placing the ball with its seam and poles in the planes, or the box in another octant of the ball, changes only where the seam and the poles meet the union.

**As a set:**

$$B_5 \cup [0,8]^3$$
