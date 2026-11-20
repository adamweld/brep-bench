+++
title = "A post less a cube beside its axis"
tags = ["boolean", "cut", "analytic", "cylinder", "coincident"]
origin = "OCCT tests/boolean/boptuc_simple/ZO6"

[volume]
value = 404.38524868566637
exact = "height 10 times the post's disc (16π) less the circular segment x > 0, 16·arccos(½) − 2·√12 = 16π/3 − 4√3"
tex = '10\,(16\pi - \tfrac{16\pi}{3} + 4\sqrt3)'

[area]
value = 317.71069023134401
exact = "wall: arc 4(2π − 2π/3) times 10 = 160π/3; the flat cut, chord 4√3 times 10 = 40√3; two ends, each 32π/3 + 4√3"
tex = '\tfrac{160\pi}{3} + 40\sqrt3 + 2(\tfrac{32\pi}{3} + 4\sqrt3)'
+++

A cube of side 10 between the origin and (10, 10, 10).

A post of radius 4 and height 10, from z = 0 to z = 10, standing beside the cube's side face x = 0: its axis is the vertical line through (−2, 5), so the post cuts across the face. Its top and bottom lie in the cube's top and bottom planes. Cut the cube away from the post.

The post's wall meets the plane x = 0 in two vertical lines at y = 5 ± √12, within the face. What is left is the post less the lens-shaped segment of it that was inside the cube: a post with a flat 4√3 wide and 10 high in place of that part of its wall.

**As a set:**

$$\{\,(x + 2)^2 + (y - 5)^2 \le 16,\ 0 \le z \le 10\,\} \setminus [0, 10]^3$$
