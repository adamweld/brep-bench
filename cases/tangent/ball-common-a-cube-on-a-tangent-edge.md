+++
title = "A ball and a cube turned about an edge that touches it, their common part"
tags = ["tangent", "boolean", "common", "analytic", "sphere", "point-contact", "seam"]
origin = "OCCT tests/bugs/modalg_4/bug7626_2"

[volume]
value = 14.660765716752369
exact = "πh²(3R − h)/3 = 14π/3, R = 5, h = 1: the cap of the ball beyond the plane 4x + 3y = 20, 4 from the center"
tex = '\tfrac{\pi h^2}{3}(3R - h) = \tfrac{14\pi}{3},\quad R = 5,\ h = 1'

[area]
value = 59.69026041820607
exact = "2πRh + πa² = 10π + 9π = 19π, the cap's sphere and its flat disk of radius a = √(R² − 4²) = 3"
tex = '2\pi R h + \pi a^2 = 10\pi + 9\pi = 19\pi,\quad a = \sqrt{R^2 - 4^2} = 3'
+++

A ball of radius 5 centered on the origin, and a cube of side 10, their common part.

Start with the cube from (5, 0, −5) to (15, 10, 5), which lies outside the ball: its edge on the
line x = 5, y = 0 touches the sphere at (5, 0, 0), where that line is tangent to it. Turn the cube
about that edge by asin(3/5) ≈ 36.87°, right-handed about +z, so that its face that was in the
plane x = 5 swings into the ball. The cube then has a corner at (5, 0, −5), and its edges from that
corner run 10 along (4/5, 3/5, 0), 10 along (−3/5, 4/5, 0) and 10 along +z.

The face that swung in lies in the plane 4x + 3y = 20, 4 from the center, and the cube holds of the
ball only the cap of height 1 beyond it; the rest of the cube is outside the ball. What is common
is that cap: a ball's cap with a flat disk of radius 3 centered at (16/5, 12/5, 0). The edge the
cube was turned about still touches the cap at (5, 0, 0), a point on the rim of its disk, and
meets it nowhere else.

Put the ball's poles on the z axis, and if your ball has a seam, put it in the half-plane y = 0,
x > 0, so that the seam passes through the point where the edge touches.

**As a set:**

$$B_5 \cap \{\,0 \le 4x + 3y - 20 \le 50,\ 0 \le -3x + 4y + 15 \le 50,\ |z| \le 5\,\}$$
