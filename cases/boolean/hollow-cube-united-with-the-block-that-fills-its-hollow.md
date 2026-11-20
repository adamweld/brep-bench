+++
title = "A cube with a closed cavity, united with the block that exactly fills the cavity"
tags = ["boolean", "union", "analytic", "planar", "coincident", "cavity"]
origin = "OCCT tests/boolean/bopfuse_simple/ZP5"

[volume]
value = 1000.0
exact = "1000: the union is the whole cube"
tex = '10^3'

[area]
value = 600.0
exact = "600: the cube's six faces; the cavity's six faces and the filler's six coincide and vanish"
tex = '6\cdot 10^2'
+++

Two solids.

The first is a cube with x, y and z from 0 to 10 with a closed cavity inside it: the box with x, y and z from 2 to 8 is hollow. It is one solid with two boundary shells, the cube's outside and the cavity's six faces, which face into the void.

The second is the solid box with x, y and z from 2 to 8, exactly the shape of the cavity. Make the union of the two.

The filler touches the first solid over the whole of the cavity's six faces, each of the filler's faces lying on the cavity's face of the same plane. The union is the solid cube, with no void left.

**As a set:**

$$\bigl([0,10]^3 \setminus (2,8)^3\bigr) \cup [2,8]^3$$
