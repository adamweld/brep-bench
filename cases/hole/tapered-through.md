+++
title = "A tapered hole through a plate, 1 in 16 on the diameter"
tags = ["hole", "taper", "cut", "planar", "cone", "analytic"]

[volume]
value = 24464.837854502888
exact = "40·40·16 − π·16/3·(5² + 5·4.5 + 4.5²) = 25600 − 1084π/3"
tex = '40\cdot 40\cdot 16 - \tfrac{\pi\cdot 16}{3}\left(5^2 + 5\cdot 4.5 + 4.5^2\right) = 25600 - \tfrac{1084\pi}{3}'

[area]
value = 6095.5981239531792
exact = "5760 − π·5² − π·4.5² + π(5 + 4.5)·√(16² + 0.5²) = 5760 − 45.25π + 4.75√1025 π: the plate, less the mouths, and the cone"
tex = '5760 - \pi\cdot 5^2 - \pi\cdot 4.5^2 + \pi\,(5 + 4.5)\sqrt{16^2 + 0.5^2} = 5760 - 45.25\pi + 4.75\sqrt{1025}\,\pi'
+++

A plate, the box from the origin to (40, 40, 16), with a tapered hole right through it, its axis
the vertical line through (20, 20). The hole is a cone about that axis: its radius is 5 at the
top face (z = 16) and 4.5 at the bottom face (z = 0), narrowing evenly between, a taper of 1 in 16
on the diameter (as a pipe thread's is). Its wall is the one cone from the top face to the bottom.

**As a set:**

$$\bigl([0,40]^2\times[0,16]\bigr) \setminus \{\,(x-20)^2 + (y-20)^2 \le (4.5 + z/32)^2\,\}$$
