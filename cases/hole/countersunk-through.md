+++
title = "A countersunk hole through a block, 90° included"
tags = ["hole", "countersink", "cut", "planar", "cylinder", "cone", "analytic"]

[volume]
value = 13947.61065788307
exact = "40·30·12 − π·3²·12 − (π·3/3·(6² + 6·3 + 3²) − π·3²·3) = 14400 − 108π − 36π = 14400 − 144π"
tex = '40\cdot 30\cdot 12 - \pi\cdot 3^2\cdot 12 - \left(\tfrac{\pi\cdot 3}{3}\left(6^2 + 6\cdot 3 + 3^2\right) - \pi\cdot 3^2\cdot 3\right) = 14400 - 144\pi'

[area]
value = 4228.232173212584
exact = "4080 − π·6² − π·3² + π(6 + 3)·3√2 + 2π·3·9 = 4080 + 9π + 27√2π: the block, less the mouths top and bottom, and the countersink's cone and the hole's wall"
tex = '4080 - \pi\cdot 6^2 - \pi\cdot 3^2 + \pi\,(6 + 3)\cdot 3\sqrt{2} + 2\pi\cdot 3\cdot 9 = 4080 + 9\pi + 27\sqrt{2}\,\pi'
+++

A block, the box from the origin to (40, 30, 12), with a countersunk hole through it, its axis
the vertical line through (20, 15).

- The hole: radius 3, right through the block from the top (z = 12) to the bottom (z = 0).
- The countersink: a cone about the same axis with an included angle of 90° (its side at 45° to
  the axis), opening into the top face in a circle of radius 6. It narrows going down until it
  meets the hole's wall in the circle of radius 3 at z = 9. So the hole's wall runs from z = 0 to
  z = 9.

If you cut the hole with a cylinder, make it run past the bottom face.

**As a set:**

$$\bigl([0,40]\times[0,30]\times[0,12]\bigr) \setminus \bigl(\{\,d \le 3\,\} \cup \{\,d \le z - 6\,\}\bigr),\quad d = \sqrt{(x-20)^2 + (y-15)^2}$$
