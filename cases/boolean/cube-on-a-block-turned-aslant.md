+++
title = "A cube resting on a block turned aslant"
tags = ["boolean", "union", "planar"]
origin = "OCCT boolean/bopfuse_simple/N5"

[volume]
value = 1316.875
exact = "5·507/8 + 10·100, where the box's plan has area 507/8 and its overlap with the cube's square, the polygon (39/5, 0); (10, 11/12); (10, 131/20); (19/2, 31/4); (1/2, 4); (13/6, 0), has area 6253/120"
tex = '5\cdot \tfrac{507}{8} + 10\cdot 100'

[area]
value = 785.0333333333333
exact = "507/8 + 100 + 5·(65/2) + 10·40 + (100 + 507/8 − 2·6253/120): the plans' perimeters of the layers, plus their floor and ceiling and the area each step between layers leaves exposed; a union plan's perimeter is the cube's 40 plus the box's less the overlap's"
tex = '\tfrac{507}{8} + 100 + 5\cdot (\tfrac{65}{2}) + 10\cdot 40 + (100 + \tfrac{507}{8} - 2\cdot \tfrac{6253}{120})'
+++

A cube of side 10 between the origin and (10, 10, 10), united with a box. The box is a rectangular block whose plan, seen from above, has the corners (3, −2), (12, 1.75), (9.5, 7.75), (0.5, 4) in order, and it is 5 high, from z = −5 to z = 0: it hangs below the cube and touches it only where its top covers part of the cube's bottom face. Its long sides, 9.75 long, run along the direction (12, 5), a turn of atan(5/12), about 22.6°, from the cube's sides; its short sides, 6.5 long, run along (−5, 12). Its corners and sides cross the cube's faces at that slant with no coincidence.

**As a set:**

$$[0,10]^3 \cup \bigl(R\times[-5,0]\bigr),\quad R = \mathrm{poly}\bigl((3, -2), (12, 1.75), (9.5, 7.75), (0.5, 4)\bigr)$$
