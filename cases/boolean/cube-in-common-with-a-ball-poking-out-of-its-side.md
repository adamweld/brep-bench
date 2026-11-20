+++
title = "A large block and a ball with a small cap outside one face, their common part"
tags = ["boolean", "common", "analytic", "sphere"]
origin = "OCCT tests/boolean/bopcommon_simple/ZP9"

[volume]
value = 508.9380098815465
exact = "4πR³/3 − πh²(3R − h)/3 with R = 5, h = 1 the height of the cap outside: 500π/3 − 14π/3"
tex = '\tfrac{4\pi R^3}{3} - \tfrac{\pi h^2(3R-h)}{3} = 162\pi'

[area]
value = 311.01767270538953
exact = "the ball less the cap's area 2πRh, plus the flat disc where the face cuts it, radius² = R² − 4² = 9: 100π − 10π + 9π"
tex = '100\pi - 2\pi R h + 9\pi = 99\pi'
+++

A block between (0, 0, 0) and (20, 20, 20), and a ball of radius 5 centred at (10, 4, 10).

The centre is 4 inside the face y = 0, so a cap of the ball 1 high sticks out through that face (the cap is bounded by a circle of radius 3 in the plane y = 0); the ball is otherwise well inside the block, touching no other face. Take the part common to the block and the ball.

The common part is the ball with that cap cut off by the plane y = 0: a ball of radius 5 with a flat circular face of radius 3.

**As a set:**

$$[0,20]^3 \cap \{(x-10)^2 + (y-4)^2 + (z-10)^2 \le 5^2\}$$
