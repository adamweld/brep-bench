+++
title = "A cube united with a block half its height, turned to a 3-4-5 slope"
tags = ["boolean", "union", "planar", "coincident"]
origin = "OCCT boolean/bopfuse_simple/L7"

[volume]
value = 1026.0416666666667
exact = "5·(100 + 50 − 1075/24) + 5·100, where the box's plan has area 50 and its overlap with the cube's square, the polygon (14/3, 0); (10, 4); (7, 8); (0, 11/4); (0, 2/3); (1/2, 0), has area 1075/24"
tex = '5\cdot (100 + 50 - \tfrac{1075}{24}) + 5\cdot 100'

[area]
value = 622.9166666666666
exact = "(100 + 50 − 1075/24) + 100 + 5·(40 + (30) − (55/2)) + 5·40 + ((100 + 50 − 1075/24) − 100): the plans' perimeters of the layers, plus their floor and ceiling and the area each step between layers leaves exposed; a union plan's perimeter is the cube's 40 plus the box's less the overlap's"
tex = '(100 + 50 - \tfrac{1075}{24}) + 100 + 5\cdot (40 + (30) - (\tfrac{55}{2})) + 5\cdot 40 + ((100 + 50 - \tfrac{1075}{24}) - 100)'
+++

A cube of side 10 between the origin and (10, 10, 10), united with a box. The box is a rectangular block whose plan, seen from above, has the corners (2, −2), (10, 4), (7, 8), (−1, 2) in order, and it is half the cube's height, from z = 0 to z = 5: its bottom lies in the cube's bottom plane and its top is inside the cube. Its long sides, 10 long, run along the direction (4, 3), a slope of 3 to 4; its short sides are 5 long. Its corner (10, 4) lies exactly on the cube's face x = 10.

**As a set:**

$$[0,10]^3 \cup \bigl(R\times[0,5]\bigr),\quad R = \mathrm{poly}\bigl((2, -2), (10, 4), (7, 8), (-1, 2)\bigr)$$
