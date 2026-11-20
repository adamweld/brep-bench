+++
title = "A wedge with a smaller wedge cut from its base"
tags = ["boolean", "cut", "planar", "pocket", "coincident", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestPrimitive.py::testPrimitiveWedge"

[volume]
value = 65
exact = "a ridged wedge on a base a × b with a ridge c long at height h is hb(2a + c)/6: 4·6·20/6 = 80 less 3·3·10/6 = 15"
tex = '\tfrac{hb(2a + c)}{6}\Big|_{(8,6,4,4)} - \Big|_{(4,3,3,2)} = 80 - 15'

[area]
value = 152.44426050800072
exact = "outer wedge 48 + 2·30 + 2·6√5 = 108 + 12√5; the opening takes 12 from the base, and the cavity's walls add 2·(9/2)√5 + 2·(3/2)√10 = 9√5 + 3√10 (its base is open, no face): 96 + 21√5 + 3√10"
tex = '\left(108 + 12\sqrt5\right) - 12 + \left(9\sqrt5 + 3\sqrt{10}\right) = 96 + 21\sqrt5 + 3\sqrt{10}'
+++

A wedge: the convex hull of the rectangle 0 ≤ x ≤ 8, 0 ≤ y ≤ 6 in the plane z = 0 and the segment
from (2, 3, 4) to (6, 3, 4), the ridge along its top. It has a flat base, two sloping trapezoid sides
(facing −y and +y) and two sloping triangular ends (facing −x and +x).

A smaller wedge of the same kind is cut away: the hull of the rectangle 2 ≤ x ≤ 6, 1.5 ≤ y ≤ 4.5 in
the plane z = 0 and the segment from (3, 3, 3) to (5, 3, 3). It stands on the same floor and lies
inside the larger one everywhere but there, touching its faces nowhere. So the result is a wedge
hollowed from below, open in a rectangle 4 by 3 in its base, its cavity a wedge ridged under the
ridge.

**As a set:**

$$\operatorname{conv}\bigl([0,8]\times[0,6]\times\{0\} \cup \{2,6\}\times\{3\}\times\{4\}\bigr) \setminus \operatorname{conv}\bigl([2,6]\times[1.5,4.5]\times\{0\} \cup \{3,5\}\times\{3\}\times\{3\}\bigr)$$
