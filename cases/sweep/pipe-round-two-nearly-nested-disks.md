+++
title = "A thin round pipe along the outline of two nearly nested disks"
tags = ["sweep", "planar-path", "corner", "round", "closed-path", "shallow-corner", "unsettled"]
origin = "OCCT pipe/standard/G1"
settled = false
+++

A pipe of circular section swept along the closed outline of the union of two disks, one almost
inside the other, their circles crossing at a shallow angle, its corners made round.

The two disks lie in the plane z = 0: the first has radius 10 and is centered on the origin, the
second has radius 9.5 and is centered on (0.6, 0, 0). The second would lie inside the first if it
were centered 0.5 or less from the origin; at 0.6 its circle pokes out through the first's by 0.1
at x = 10.1, and the two circles cross at two points, (8.425, ±5.387, 0) to three places. The path
is the outline of the union of the two disks: the arc of the first circle outside the second disk
(the long way round, through (−10, 0, 0)) and the short arc of the second circle outside the first
disk (through (10.1, 0, 0)), joined end to end at the two crossing points, where the outline has
two corners, each a shallow turn of about 2°, 10.8 apart.

The pipe's section is a disk of radius 0.25, centered on the path and square to it. At each corner
the section is turned about the corner (the corner made round), so the pipe is every point within
0.25 of the outline: a ring.

**As a set:**

$$\{\,p : d(p, \gamma) \le 0.25\,\},\quad \gamma = \partial(D_1 \cup D_2),\ D_1 = \{x^2 + y^2 \le 100,\ z = 0\},\
D_2 = \{(x - 0.6)^2 + y^2 \le 9.5^2,\ z = 0\}$$
