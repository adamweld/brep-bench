+++
title = "A hole drilled through a plate at 30° from square"
tags = ["hole", "inclined", "cut", "planar", "cylinder", "ellipse", "analytic"]

[volume]
value = 23419.58420344505
exact = "60·40·10 − π·4²·10/cos 30° = 24000 − 320π/√3: every slice of the hole by a plane z = c is an ellipse of area π·4²/cos 30°"
tex = '60\cdot 40\cdot 10 - \frac{\pi\cdot 4^2\cdot 10}{\cos 30^\circ} = 24000 - \frac{320\pi}{\sqrt{3}}'

[area]
value = 6974.1247389664849
exact = "6800 − 2·π·4²/cos 30° + 2π·4·10/cos 30° = 6800 + 32√3π: the plate, less the two elliptic mouths, and the hole's wall, each of whose straight lines runs 10/cos 30° between the faces"
tex = '6800 - 2\cdot\frac{\pi\cdot 4^2}{\cos 30^\circ} + \frac{2\pi\cdot 4\cdot 10}{\cos 30^\circ} = 6800 + 32\sqrt{3}\,\pi'
+++

A plate, the box from the origin to (60, 40, 10), with a round hole of radius 4 drilled right
through it at a slant. The hole's axis passes through the plate's middle, (30, 20, 5), and lies in
the plane y = 20, leaning 30° from the vertical toward +x: its direction is (sin 30°, 0, cos 30°).
So the hole comes out of the top face (z = 10) around (30 + 5 tan 30°, 20) ≈ (32.887, 20) and out of
the bottom face around (27.113, 20), each mouth an ellipse with semi-axes 4/cos 30° along x and 4
along y.

If you cut the hole with a cylinder, make it run past both faces.

**As a set:**

$$\bigl([0,60]\times[0,40]\times[0,10]\bigr) \setminus \{\,\bigl((x-30)\cos 30^\circ - (z-5)\sin 30^\circ\bigr)^2 + (y-20)^2 \le 4^2\,\}$$
