+++
title = "A NURBS ball less the octant through its pole and seam"
tags = ["nurbs", "rational", "boolean", "cut", "analytic", "pole", "seam"]
origin = "OCCT tests/boolean/bcut_simple/A1"

[volume]
value = 458.1489286485115
exact = "seven eighths of the ball, 7/8·4πr³/3 = 875π/6, r = 5"
tex = '\tfrac78\cdot\tfrac43\pi r^3 = \tfrac{875\pi}{6},\quad r = 5'

[area]
value = 333.79421944391555
exact = "seven eighths of the sphere and three quarter disks, 7/8·4πr² + 3·πr²/4 = 175π/2 + 75π/4 = 425π/4"
tex = '\tfrac78\cdot 4\pi r^2 + 3\cdot\tfrac{\pi r^2}{4} = \tfrac{425\pi}{4}'
+++

A ball of radius 5 centered on the origin, bounded by one rational B-spline surface (a NURBS), the
standard biquadratic sphere of `geometry/nurbs-sphere`: a 9 × 5 net whose first and last rows each
collapse to a pole. Cut from it the box from (0, 0, 0) to (8, 8, 8).

The surface is the tensor product of two rational quadratic curves. Around the axis (u), the unit
circle of nine control points cᵢ = (1, 0), (1, 1), (0, 1), (−1, 1), (−1, 0), (−1, −1), (0, −1),
(1, −1), (1, 0), with weights wᵢ = 1, √2/2, 1, √2/2, 1, √2/2, 1, √2/2, 1, on the knots
(0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1). Along the meridian (v), the semicircle from the south pole to
the north of five control points (ρⱼ, zⱼ) = (0, −5), (5, −5), (5, 0), (5, 5), (0, 5), with weights
vⱼ = 1, √2/2, 1, √2/2, 1, on the knots (0, 0, 0, ½, ½, 1, 1, 1). The net's control point (i, j) is
(ρⱼ cᵢₓ, ρⱼ cᵢᵧ, zⱼ), with weight wᵢvⱼ. So the poles are (0, 0, ±5), and the seam, where the
surface's edges u = 0 and u = 1 meet, is the meridian in the half-plane y = 0, x ≥ 0. Do not use a
sphere.

The box's corner at the center is inside the ball and its other seven corners are outside, so it
takes the octant x, y, z ≥ 0 of the ball, and seven eighths of the ball remain. The cut runs through
the surface's special curves: the box's faces x = 0 and y = 0 meet along the z axis, through the
north pole, where the surface's whole edge v = 1 collapses to a point, and the face y = 0 holds the
seam's upper half, from (5, 0, 0) to the north pole. The result is bounded by the NURBS surface less
that octant and by three flat quarter disks of radius 5, one in each of the planes x = 0, y = 0 and
z = 0.

**As a set:**

$$B_5 \setminus [0,8]^3$$
