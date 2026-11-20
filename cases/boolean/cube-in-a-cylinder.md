+++
title = "A cube in a cylinder"
tags = ["boolean", "common", "cylinder", "analytic"]
origin = "OCCT boolean/gdml_public/A3, boolean/gdml_public/A4"

[volume]
value = 497.51764368333113
exact = "8·A, A = π·5² − 4(5²α − 4·3), α = arccos(4/5): the section is the disk less four segments, each of a chord 6"
tex = '8A,\quad A = \pi\cdot 5^2 - 4\left(5^2\alpha - 4\cdot 3\right),\ \alpha = \arccos\tfrac45'

[area]
value = 361.78646839416524
exact = "top and bottom 2A, four flat faces 6·8 each, and the four arcs of the side 5·8·(2π − 8α)"
tex = '2A + 4\cdot 6\cdot 8 + 5\cdot 8\left(2\pi - 8\alpha\right)'
+++

The part common to a cube and a cylinder. The cube is the box from (−4, −4, 0) to (4, 4, 8). The cylinder has radius 5 and lies on the z axis, running from z = −2 to z = 10, past both of the cube's faces.

The cylinder's radius is more than the cube's half-width 4 and less than its half-diagonal 4√2 ≈ 5.66, so it cuts off each of the cube's four vertical edges and leaves the middle of each side face, a strip 6 wide. The solid has the two flat ends, four flat strips and four arcs of the cylinder, one at each edge.

**As a set:**

$$\{\,|x| \le 4,\ |y| \le 4,\ 0 \le z \le 8,\ \rho \le 5\,\}$$
