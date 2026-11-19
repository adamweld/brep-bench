+++
title = "A blind drilled hole ending in a 118° drill point"
tags = ["hole", "blind", "drill-point", "cut", "planar", "cylinder", "cone", "analytic"]

[volume]
value = 21856.544145303967
exact = "30·30·25 − π·4²·12 − (1/3)π·4²·h, h = 4 cot 59°: = 22500 − 192π − (64/3)π cot 59°"
tex = '30\cdot 30\cdot 25 - \pi\cdot 4^2\cdot 12 - \tfrac{1}{3}\pi\cdot 4^2\cdot 4\cot 59^\circ = 22500 - 192\pi - \tfrac{64}{3}\pi\cot 59^\circ'

[area]
value = 5109.9688028491704
exact = "4800 − π·4² + 2π·4·12 + π·4·(4 csc 59°) = 4800 + 80π + 16π csc 59°: the block, less the mouth, and the hole's wall and the cone of its point"
tex = '4800 - \pi\cdot 4^2 + 2\pi\cdot 4\cdot 12 + \pi\cdot 4\cdot 4\csc 59^\circ = 4800 + 80\pi + 16\pi\csc 59^\circ'
+++

A block, the box from the origin to (30, 30, 25), with a blind hole drilled down into it from the
top face, its axis the vertical line through (15, 15).

The hole has radius 4. Its wall runs down from the top face (z = 25) at its full radius for 12, to
z = 13. There it ends in the point a twist drill leaves: a cone about the axis with an included
angle of 118° (its side at 59° to the axis), its base the circle of radius 4 at z = 13 and its apex
on the axis below, at z = 13 − 4 cot 59° ≈ 10.597. The hole does not reach the bottom face.

**As a set:**

$$\bigl([0,30]^2\times[0,25]\bigr) \setminus \{\,d \le 4,\ d \le 4 + (z - 13)\tan 59^\circ\,\},\quad d = \sqrt{(x-15)^2 + (y-15)^2}$$
