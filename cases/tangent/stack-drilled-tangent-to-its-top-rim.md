+++
title = "A stack drilled through, the drill's wall touching the cone's top rim"
tags = ["boolean", "cut", "analytic", "cylinder", "cone", "tangent", "seam"]
origin = "OCCT boolean/bcut_simple/G9"

[volume]
value = 1551.9467708733578
exact = "(400π + 104π) − 10π"
tex = '(400\pi + 104\pi) - 10\pi'

[area]
value = 1016.8984202411086
exact = "248π + 16√13π − 2π + 20π"
tex = '248\pi + 16\sqrt{13}\,\pi - 2\pi + 20\pi'
+++

A stack of two solids of revolution about the z axis: a disc, the cylinder of radius 10 from z = 0 to z = 4, and on it a cone frustum from z = 4 to z = 10, its radius falling straight from 6 at the bottom to 2 at the top, joined to the disc by its whole bottom face. Through it a thin drill is bored: the cylinder of radius 1 whose axis is the vertical line through (1, 0), running from below the stack to above it. Cut the drill from the stack.

The drill's disk, centred on (1, 0) with radius 1, lies inside the frustum's top disk of radius 2 and touches its rim at the single point (2, 0, 10), where the drill's wall meets the cone's top rim; everywhere else the wall stays inside the cone, the cone's wall thinning to nothing as the rim is approached. The top face is left as a crescent that closes to a point there.

**As a set:**

$$\bigl(\{\,\rho \le 10,\ 0 \le z \le 4\,\} \cup \{\,4 \le z \le 10,\ \rho \le 6 - \tfrac23 (z - 4)\,\}\bigr) \setminus \{\,(x - 1)^2 + y^2 \le 1\,\}$$
