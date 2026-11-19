+++
title = "A ball less a cube turned about an edge that touches it"
tags = ["tangent", "boolean", "cut", "analytic", "sphere", "point-contact", "seam"]
origin = "OCCT tests/bugs/modalg_4/bug7626_1"

[volume]
value = 508.9380098815465
exact = "4πR³/3 − πh²(3R − h)/3 = 500π/3 − 14π/3 = 162π, R = 5, h = 1: of the cube only the cap beyond the plane 4x + 3y = 20, 4 from the center, is in the ball"
tex = '\tfrac{4}{3}\pi R^3 - \tfrac{\pi h^2}{3}(3R - h) = \tfrac{500\pi}{3} - \tfrac{14\pi}{3} = 162\pi,\quad R = 5,\ h = 1'

[area]
value = 311.01767270538954
exact = "4πR² − 2πRh + πa² = 100π − 10π + 9π = 99π, the cap's sphere replaced by its flat disk of radius a = √(R² − 4²) = 3"
tex = '4\pi R^2 - 2\pi R h + \pi a^2 = 100\pi - 10\pi + 9\pi = 99\pi,\quad a = \sqrt{R^2 - 4^2} = 3'
+++

A ball of radius 5 centered on the origin, less a cube of side 10.

Start with the cube from (5, 0, −5) to (15, 10, 5), which lies outside the ball: its edge on the
line x = 5, y = 0 touches the sphere at (5, 0, 0), where that line is tangent to it. Turn the cube
about that edge by asin(3/5) ≈ 36.87°, right-handed about +z, so that its face that was in the
plane x = 5 swings into the ball. The cube then has a corner at (5, 0, −5), and its edges from that
corner run 10 along (4/5, 3/5, 0), 10 along (−3/5, 4/5, 0) and 10 along +z. Cut it from the ball.

The face that swung in lies in the plane 4x + 3y = 20, 4 from the center, and the cube takes from
the ball the cap of height 1 beyond it; the rest of the cube is outside the ball. What remains is the
ball cut flat by that plane, its flat face the disk of radius 3 centered at (16/5, 12/5, 0). The
edge the cube was turned about still touches the ball at (5, 0, 0), a point on the rim of that disk,
and meets the result nowhere else.

Put the ball's poles on the z axis, and if your ball has a seam, put it in the half-plane y = 0,
x > 0, so that the seam passes through the point where the edge touches.

**As a set:**

$$B_5 \setminus \{\,0 \le 4x + 3y - 20 \le 50,\ 0 \le -3x + 4y + 15 \le 50,\ |z| \le 5\,\}$$
