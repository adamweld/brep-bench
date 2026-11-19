+++
title = "A hole spotfaced flat into a sloping face"
tags = ["hole", "spotface", "counterbore", "cut", "planar", "cylinder", "ellipse", "analytic"]

[volume]
value = 35790.973596447152
exact = "1250·30 − π·8²·3 − π·4²·22 = 37500 − 544π: the spotface's depth above its floor is linear across its disk, so its mean is the depth on the axis, 30 − 25/5 − 22 = 3"
tex = '1250\cdot 30 - \pi\cdot 8^2\cdot 3 - \pi\cdot 4^2\cdot 22 = 37500 - 544\pi'

[area]
value = 7628.9098326683221
exact = "5500 + 300√26 − π·8²·√26/5 + 2π·8·3 + π(8² − 4²) + 2π·4·22 − π·4² = 5500 + 300√26 + 256π − 12.8√26π: the block, less the spotface's mouth in the slope (its disk over the cosine of the slope), and the spotface's wall (its mean height 3) and floor, the hole's wall and less its mouth in the bottom"
tex = '5500 + 300\sqrt{26} - \frac{\pi\cdot 8^2\sqrt{26}}{5} + 2\pi\cdot 8\cdot 3 + \pi\left(8^2 - 4^2\right) + 2\pi\cdot 4\cdot 22 - \pi\cdot 4^2 = 5500 + 300\sqrt{26} + 256\pi - 12.8\sqrt{26}\,\pi'
+++

A block with a sloping top, and a vertical hole through it whose seat is spotfaced flat into the
slope, so that a bolt's head bears square to it.

The block: the polygon through (x, z) = (0, 0), (50, 0), (50, 20), (0, 30) in the plane y = 0,
extruded along +y to y = 30. Its top is a slope falling from z = 30 at x = 0 to z = 20 at x = 50,
the plane z = 30 − x/5.

Both features are about the vertical axis through (25, 15):

- the spotface: radius 8, cut down from the slope to a flat, level floor at z = 22. The slope
  crosses its wall between z = 23.4 (at x = 33) and z = 26.6 (at x = 17), so the wall is everywhere
  above the floor, and its rim in the slope is an ellipse;
- the hole: radius 4, from the spotface's floor right through the block to the bottom face (z = 0).

If you cut the hole with a cylinder, make it run past the bottom face.

**As a set:**

$$\{\,0 \le x \le 50,\ 0 \le y \le 30,\ 0 \le z \le 30 - x/5\,\} \setminus \bigl(\{\,d \le 4\,\} \cup \{\,d \le 8,\ z \ge 22\,\}\bigr),\quad d = \sqrt{(x-25)^2 + (y-15)^2}$$
