+++
title = "A box sliced aslant into three solids"
tags = ["split", "multi-body", "planar", "analytic"]
origin = "FreeCAD src/Mod/Part/parttests/TopoShapeTest.py testTopoShapeSlices"

[volume]
value = 6000.0
exact = "30·20·10, the three pieces together: (15³ − 5³)/6 = 1625/3, 6875/2 and (25³ − 15³ − 5³)/6 = 12125/6"
tex = '30\cdot 20\cdot 10 = \tfrac{1625}{3} + \tfrac{6875}{2} + \tfrac{12125}{6}'

[area]
value = 3195.9292143521043
exact = "the box 2(600 + 300 + 200) and each cut twice, once for each piece beside it: 2√3·100 + 2√3·187.5 = 2200 + 575√3 (a cut is √3 times its shadow on z = 0, the part of the rectangle where 5 ≤ x + y ≤ 15, or 25 ≤ x + y ≤ 35)"
tex = '2\left(600 + 300 + 200\right) + 2\sqrt{3}\left(100 + 187.5\right) = 2200 + 575\sqrt{3}'
+++

A box from the origin to (30, 20, 10), split by the two parallel planes x + y + z = 15 and
x + y + z = 35, square to the direction (1, 1, 1), into three solids, kept as three. They are the
piece where x + y + z ≤ 15, the slab between the planes, and the piece where x + y + z ≥ 35. They
touch along the cuts but are not joined: each cut is a face of both the pieces beside it.

The first cut is the quadrilateral (5, 0, 10), (15, 0, 0), (0, 15, 0), (0, 5, 10), and the second
the pentagon (25, 0, 10), (30, 0, 5), (30, 5, 0), (15, 20, 0), (5, 20, 10). The pieces' volumes
are 1625/3, 6875/2 and 12125/6.

**As a set:**

$$K \cap \{\,x + y + z \le 15\,\},\quad K \cap \{\,15 \le x + y + z \le 35\,\},\quad K \cap \{\,x + y + z \ge 35\,\}$$

K = [0, 30] × [0, 20] × [0, 10], and each of the three is a solid of its own.
