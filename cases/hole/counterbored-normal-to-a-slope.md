+++
title = "A counterbored hole square to a sloping face, coming out aslant"
tags = ["hole", "counterbore", "inclined", "cut", "planar", "cylinder", "ellipse", "analytic"]

[volume]
value = 87832.301069023043
exact = "2250·40 − π·7²·5 − π·4²·L, L = 26.25/0.8 − 5 = 27.8125 (the hole's length on its axis, from the counterbore's floor to the bottom face): = 90000 − 690π"
tex = '2250\cdot 40 - \pi\cdot 7^2\cdot 5 - \pi\cdot 4^2\cdot L,\quad L = \tfrac{26.25}{0.8} - 5 = 27.8125:\ = 90000 - 690\pi'

[area]
value = 13705.818515645782
exact = "12900 − π·7² + π(7² − 4²) + 2π·7·5 + 2π·4·L − π·4²/0.8 = 12900 + 256.5π, with L as for the volume: the wedge (4500 + 2400 + 2400 + 600 + 75·40), less the counterbore's mouth, and its floor, its wall, the hole's wall and less the elliptic mouth in the bottom"
tex = '12900 - \pi\cdot 7^2 + \pi\left(7^2 - 4^2\right) + 2\pi\cdot 7\cdot 5 + 2\pi\cdot 4\cdot L - \frac{\pi\cdot 4^2}{0.8} = 12900 + 256.5\pi'
+++

A wedge-shaped block with a counterbored hole drilled square to its sloping face.

The block: the polygon through (x, z) = (0, 0), (60, 0), (60, 15), (0, 60) in the plane y = 0,
extruded along +y to y = 40. Its top is a slope falling from z = 60 at x = 0 to z = 15 at x = 60,
the plane 0.6x + 0.8z = 48, whose outward normal is (0.6, 0, 0.8).

The hole: its axis is the line square to the slope through the point (45, 20, 26.25) on it, running
into the block along −(0.6, 0, 0.8).

- The counterbore: radius 7 and 5 deep, measured along the axis from the slope, with a flat floor
  square to the axis.
- The hole: radius 4, from the counterbore's floor on along the axis right through the block. It
  comes out of the bottom face (z = 0) aslant, the axis meeting it at (25.3125, 20, 0), and its
  mouth there is an ellipse with semi-axes 5 along x and 4 along y.

If you cut the hole with a cylinder, make it run past the bottom face.

**As a set:** with $s = 0.6x + 0.8z - 48$, the distance along the axis out of the slope, and
$q = \bigl(0.8(x-45) - 0.6(z-26.25)\bigr)^2 + (y-20)^2$, the square of the distance from the axis,

$$\{\,0 \le x \le 60,\ 0 \le y \le 40,\ z \ge 0,\ s \le 0\,\} \setminus \bigl(\{\,q \le 4^2\,\} \cup \{\,q \le 7^2,\ s \ge -5\,\}\bigr)$$
