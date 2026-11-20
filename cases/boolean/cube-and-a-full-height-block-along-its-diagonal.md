+++
title = "A cube united with a block as tall as itself, laid along its diagonal"
tags = ["boolean", "union", "planar", "coincident"]
origin = "OCCT boolean/bopfuse_simple/C3"

[volume]
value = 1180.0
exact = "10·(100 + 60 − 42), where the box's plan has area 60 and its overlap with the cube's square, the polygon (0, 0); (10, 10); (4, 10); (0, 6), has area 42"
tex = '10\cdot (100 + 60 - 42)'

[area]
value = 685.7056274847714
exact = "(100 + 60 − 42) + (100 + 60 − 42) + 10·(40 + (26·√2) − (12 + 14·√2)): the plans' perimeters of the layers, plus their floor and ceiling and the area each step between layers leaves exposed; a union plan's perimeter is the cube's 40 plus the box's less the overlap's"
tex = '(100 + 60 - 42) + (100 + 60 - 42) + 10\cdot (40 + (26\cdot \sqrt{2}) - (12 + 14\cdot \sqrt{2}))'
+++

A cube of side 10 between the origin and (10, 10, 10), united with a box. The box is a rectangular block whose plan, seen from above, has the corners (0, 0), (10, 10), (7, 13), (−3, 3) in order, and it is as tall as the cube, from z = 0 to z = 10, so its top lies in the cube's top plane and its bottom in the cube's bottom plane. Its long sides run along the diagonal of the cube's plan: one of them from the cube's corner (0, 0) to its opposite corner (10, 10), the other 3√2 away on the outside. Its short sides are 3√2 long.

**As a set:**

$$[0,10]^3 \cup \bigl(R\times[0,10]\bigr),\quad R = \mathrm{poly}\bigl((0, 0), (10, 10), (7, 13), (-3, 3)\bigr)$$
