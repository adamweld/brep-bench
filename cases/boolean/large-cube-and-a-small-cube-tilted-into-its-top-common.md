+++
title = "A large cube and a small cube tilted with its lower edge just under the large one's top face, their common part"
tags = ["boolean", "common", "analytic", "planar", "general-position"]
origin = "OCCT tests/boolean/bopcommon_simple/Q9"

[volume]
value = 4.618802153517007
exact = "the wedge is a prism 4 long on a triangle with a right angle at the buried edge, legs 2 and 2/√3: 4·(1/2)·2·(2/√3) = 8/√3"
tex = '\frac{8}{\sqrt3}'

[area]
value = 24.165807537309522
exact = "the top face 4 × 4/√3, the two walls 4 × 2 and 4 × 2/√3, and two triangular ends of 2/√3 each: 16/√3 + 8 + 8/√3 + 4/√3"
tex = '8 + \frac{28}{\sqrt3}'
+++

A cube from (0, 0, 0) to (20, 20, 20), and a small cube of side 4 that begins as the cube from (8, 8, 19) to (12, 12, 23) and is turned 30° about the line through (8, 8, 19) parallel to the x axis (right-handed about +x, so that the small cube's faces through that line lean up and toward +y and −y: one face rises toward +y at 30° above horizontal, the other toward −y at 60°). The small cube's lowest edge, the line y = 8, z = 19 from x = 8 to x = 12, is buried 1 below the large cube's top face z = 20, and the rest of the small cube stands above it, except for a wedge under the plane z = 20. Make the solid common to both.

The common part is that wedge: a prism 4 long (x from 8 to 12) on a triangle in the plane x = const with its right angle at the buried edge (8, 19), whose legs are 2, along (cos 30°, sin 30°), and 2/√3, along (−sin 30°, cos 30°), where they reach z = 20. Its top is a rectangle 4 by 4/√3 in the large cube's top face.

**As a set:**

$$[0,20]^3 \cap R_\ell(30^\circ)\bigl([8,12]\times[8,12]\times[19,23]\bigr),\quad \ell = \{y = 8,\ z = 19\}$$
