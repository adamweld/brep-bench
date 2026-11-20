+++
title = "A ball cut by a thick slab lying aslant, off its centre"
tags = ["boolean", "cut", "analytic", "sphere", "planar", "off-axis"]
origin = "OCCT bugs/modalg_7/bug32470"

[volume]
value = 297.4041045398338
exact = "πh²(3R − h)/3 for the two caps, R = 5, h = 3 and 4: 36π + 176π/3"
tex = '\tfrac{\pi}{3}h^2(3R - h),\quad R = 5,\ h = 3,\ 4:\quad 36\pi + \tfrac{176\pi}{3}'

[area]
value = 361.28315516282623
exact = "the caps' spheres 2πRh = 30π and 40π, and their flat faces π(R² − d²) = 21π and 24π, d = 2 and 1"
tex = '30\pi + 40\pi + 21\pi + 24\pi'
+++

A ball of radius 5 about the origin, less a thick slab turned aslant to the axes. The slab is the flat box with its two big faces in the planes $n \cdot p = -1$ and $n \cdot p = 2$, where $n = (1, 2, 2)/3$, so it is 3 thick and its faces lie 1 and 2 from the ball's centre, on opposite sides of it. It is a box 30 by 30 in those planes, centred on the point $0.5\,n$, with sides along $u = (2, -2, 1)/3$ and $v = (2, 1, -2)/3$, which with $n$ make a right-handed frame. It reaches well past the ball in every direction across its faces, so it cuts the ball in two.

What is left is two caps of the ball, a small one 3 high beyond the plane $n \cdot p = 2$ and a bigger one 4 high beyond $n \cdot p = -1$, apart from each other. Measure the two together as one result.

**As a set:**

$$\{\,\lVert p \rVert \le 5\,\} \setminus \{\,-1 \le n \cdot p \le 2\,\}, \quad n = (1, 2, 2)/3$$
