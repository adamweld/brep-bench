+++
title = "A cube united with a block turned aslant and raised through its top"
tags = ["boolean", "union", "planar"]
origin = "OCCT boolean/bopfuse_simple/P1"

[volume]
value = 1373.2083333333333
exact = "5·100 + 5·(100 + 507/8 − 6253/120) + 5·507/8, where the box's plan has area 507/8 and its overlap with the cube's square, the polygon (39/5, 0); (10, 11/12); (10, 131/20); (19/2, 31/4); (1/2, 4); (13/6, 0), has area 6253/120"
tex = '5\cdot 100 + 5\cdot (100 + \tfrac{507}{8} - \tfrac{6253}{120}) + 5\cdot \tfrac{507}{8}'

[area]
value = 802.3666666666667
exact = "100 + 507/8 + 5·40 + 5·(40 + (65/2) − (871/30)) + 5·(65/2) + ((100 + 507/8 − 6253/120) − 100) + ((100 + 507/8 − 6253/120) − 507/8): the plans' perimeters of the layers, plus their floor and ceiling and the area each step between layers leaves exposed; a union plan's perimeter is the cube's 40 plus the box's less the overlap's"
tex = '100 + \tfrac{507}{8} + 5\cdot 40 + 5\cdot (40 + (\tfrac{65}{2}) - (\tfrac{871}{30})) + 5\cdot (\tfrac{65}{2}) + ((100 + \tfrac{507}{8} - \tfrac{6253}{120}) - 100) + ((100 + \tfrac{507}{8} - \tfrac{6253}{120}) - \tfrac{507}{8})'
+++

A cube of side 10 between the origin and (10, 10, 10), united with a box. The box is a rectangular block whose plan, seen from above, has the corners (3, −2), (12, 1.75), (9.5, 7.75), (0.5, 4) in order, and it is 10 high, from z = 5 to z = 15: it starts inside the cube, leaves through the cube's top face and stands 5 above it. Its long sides, 9.75 long, run along the direction (12, 5), a turn of atan(5/12), about 22.6°, from the cube's sides; its short sides, 6.5 long, run along (−5, 12). Its corners and sides cross the cube's faces at that slant with no coincidence.

**As a set:**

$$[0,10]^3 \cup \bigl(R\times[5,15]\bigr),\quad R = \mathrm{poly}\bigl((3, -2), (12, 1.75), (9.5, 7.75), (0.5, 4)\bigr)$$
