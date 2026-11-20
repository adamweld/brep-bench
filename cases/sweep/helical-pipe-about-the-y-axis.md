+++
title = "A helical pipe wound about the y axis"
tags = ["sweep", "helix", "analytic"]
origin = "OCCT tests/helix/standard/D6"

[volume]
value = 395.486676010665
exact = "πr²L, L = n√((2πR)² + p²), r = 1, R = 8, p = 3, n = 5/2: a tube's volume is its section's area times its length, wherever its radius stays below the path's least radius of curvature (here about 8)"
tex = '\pi r^2 L,\quad L = n\sqrt{(2\pi R)^2 + p^2},\quad r = 1,\ R = 8,\ p = 3,\ n = \tfrac52'

[area]
value = 797.2565373285108
exact = "2πrL + 2πr²: the tube's side and its two flat ends"
tex = '2\pi r L + 2\pi r^2'
+++

A pipe of circular section, radius 1, following a helix whose axis is the y axis, not the z axis.
The helix has radius 8 and rises 3 per turn, along +y. It starts at (0, 0, 8), turns right-handed
about +y (from +z toward +x) and makes two and a half turns, so it ends at (0, 7.5, −8). The section
stays square to the path, and the pipe's ends are flat, square to the helix there.

Neighboring coils are 3 apart, centre to centre, so the pipe, 2 across, never touches itself.

**As a set:**

$$\bigl\{\,c(t) + u : 0 \le t \le 5\pi,\ u \perp c'(t),\ \lVert u \rVert \le 1\,\bigr\},
\quad c(t) = \bigl(8\sin t,\ \tfrac{3t}{2\pi},\ 8\cos t\bigr)$$
