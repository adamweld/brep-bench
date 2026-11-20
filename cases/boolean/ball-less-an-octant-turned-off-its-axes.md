+++
title = "A ball less an octant, the ball turned off the axes"
tags = ["boolean", "cut", "analytic", "sphere", "planar", "pole", "seam"]
origin = "OCCT boolean/bcut_simple/A2, OCCT boolean/bcut_simple/A4"

[volume]
value = 458.1489286485115
exact = "seven eighths of the ball, 7/8·4πr³/3 = 875π/6, r = 5"
tex = '\tfrac78\cdot\tfrac43\pi r^3 = \tfrac{875\pi}{6},\quad r = 5'

[area]
value = 333.79421944391555
exact = "7/8·4πr² + 3·πr²/4 = 175π/2 + 75π/4 = 425π/4"
tex = '\tfrac78\cdot 4\pi r^2 + 3\cdot\tfrac{\pi r^2}{4} = \tfrac{425\pi}{4}'
+++

A ball of radius 5 centered on the origin, bounded by a plain spherical surface (not a B-spline): the sphere with its axis along (1, 2, 2)/3 and its seam, the meridian where the surface's angle about the axis wraps, in the half-plane through that axis and the direction (2, 1, −2)/3. So its poles are at ±(5/3, 10/3, 10/3), and neither the poles nor the seam lie in any of the planes x = 0, y = 0 or z = 0. Cut from it the box from (0, 0, 0) to (8, 8, 8).

The box's corner at the center is inside the ball and its other seven corners are outside, so it takes the octant x, y, z ≥ 0 of the ball, and seven eighths of the ball remain. The result is bounded by the sphere less that octant and by three flat quarter disks of radius 5, one in each of the planes x = 0, y = 0 and z = 0. Placing the box in another octant of the ball, say from (−8, 0, 0) to (0, 8, 8), changes only where the seam and the poles meet the cut.

**As a set:**

$$B_5 \setminus [0,8]^3$$
