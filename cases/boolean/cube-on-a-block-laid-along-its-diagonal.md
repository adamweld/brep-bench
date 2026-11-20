+++
title = "A cube resting on a block laid along its diagonal"
tags = ["boolean", "union", "planar", "coincident"]
origin = "OCCT boolean/bopfuse_simple/N9"

[volume]
value = 1300.0
exact = "5·60 + 10·100, where the box's plan has area 60 and its overlap with the cube's square, the polygon (0, 0); (10, 10); (4, 10); (0, 6), has area 42"
tex = '5\cdot 60 + 10\cdot 100'

[area]
value = 819.8477631085024
exact = "60 + 100 + 5·(26·√2) + 10·40 + (100 + 60 − 2·42): the plans' perimeters of the layers, plus their floor and ceiling and the area each step between layers leaves exposed; a union plan's perimeter is the cube's 40 plus the box's less the overlap's"
tex = '60 + 100 + 5\cdot (26\cdot \sqrt{2}) + 10\cdot 40 + (100 + 60 - 2\cdot 42)'
+++

A cube of side 10 between the origin and (10, 10, 10), united with a box. The box is a rectangular block whose plan, seen from above, has the corners (0, 0), (10, 10), (7, 13), (−3, 3) in order, and it is 5 high, from z = −5 to z = 0: it hangs below the cube and touches it only where its top covers part of the cube's bottom face. Its long sides run along the diagonal of the cube's plan: one of them from the cube's corner (0, 0) to its opposite corner (10, 10), the other 3√2 away on the outside. Its short sides are 3√2 long.

**As a set:**

$$[0,10]^3 \cup \bigl(R\times[-5,0]\bigr),\quad R = \mathrm{poly}\bigl((0, 0), (10, 10), (7, 13), (-3, 3)\bigr)$$
