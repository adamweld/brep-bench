+++
title = "A counterdrilled hole: a wide bore stepping down to a narrow one by a 118° cone"
tags = ["hole", "counterdrill", "drill-point", "cut", "planar", "cylinder", "cone", "analytic"]

[volume]
value = 17096.735784709299
exact = "30·30·20 − π·2.5²·20 − π(5² − 2.5²)·8 − (π·h/3·(5² + 5·2.5 + 2.5²) − π·2.5²·h), h = 2.5 cot 59°: = 18000 − 275π − (125/6)π cot 59°"
tex = '30\cdot 30\cdot 20 - \pi\cdot 2.5^2\cdot 20 - \pi\left(5^2 - 2.5^2\right)\cdot 8 - \left(\tfrac{\pi h}{3}\left(5^2 + 5\cdot 2.5 + 2.5^2\right) - \pi\cdot 2.5^2\,h\right),\ h = 2.5\cot 59^\circ:\ = 18000 - 275\pi - \tfrac{125}{6}\pi\cot 59^\circ'

[area]
value = 4586.7728393106135
exact = "4200 − π·5² − π·2.5² + 2π·5·8 + π(5 + 2.5)·2.5 csc 59° + 2π·2.5·(12 − 2.5 cot 59°) = 4200 + 108.75π − 12.5π cot 59° + 18.75π csc 59°"
tex = '4200 - \pi\cdot 5^2 - \pi\cdot 2.5^2 + 2\pi\cdot 5\cdot 8 + \pi\,(5 + 2.5)\cdot 2.5\csc 59^\circ + 2\pi\cdot 2.5\left(12 - 2.5\cot 59^\circ\right) = 4200 + 108.75\pi - 12.5\pi\cot 59^\circ + 18.75\pi\csc 59^\circ'
+++

A block, the box from the origin to (30, 30, 20), with a counterdrilled hole through it, its axis
the vertical line through (15, 15). Seen from the top, going down:

- a bore of radius 5, at its full radius from the top face (z = 20) down to z = 12;
- then a cone about the axis with an included angle of 118° (its side at 59° to the axis), the
  point of the drill that made the bore, narrowing from radius 5 at z = 12 until it meets the
  narrow hole's wall at radius 2.5, at z = 12 − 2.5 cot 59° ≈ 10.498;
- then the narrow hole, radius 2.5, on down through the bottom face (z = 0).

If you cut the narrow hole with a cylinder, make it run past the bottom face.

**As a set:**

$$\bigl([0,30]^2\times[0,20]\bigr) \setminus \bigl(\{\,d \le 2.5\,\} \cup \{\,d \le 5,\ d \le 5 + (z - 12)\tan 59^\circ\,\}\bigr),\quad d = \sqrt{(x-15)^2 + (y-15)^2}$$
