+++
title = "A notched prism, its concave edge between planes at an angle chamfered by its throat"
tags = ["chamfer", "throat", "concave", "plane", "prism"]
origin = "OCCT tests/bugs/modalg_7/bug28828_10"

[volume]
value = 324.0
exact = "8·(40 + 1/2): the profile's area 40 and the triangle the chamfer fills, ½d² sin θ = ½·(5/4)·(4/5) = 1/2, times the length 8"
tex = '8\left(40 + \tfrac12\right)'

[area]
value = 335.11145618000168
exact = "2·(40 + 1/2) + 8·(34 − √5): the two end faces and the profile's outline, 34 − 2d with d = √5/2, the legs the chamfer takes"
tex = '81 + 8\left(34 - \sqrt{5}\right)'
+++

In the xz plane, take the polygon through (0, 0), (10, 0), (10, 3), (4, 3), (1, 7) and (0, 7), and
extrude it along +y from y = 0 to y = 8. The prism has one concave edge, the line x = 4, z = 3 along
y, where the face z = 3 (from x = 4 to x = 10) meets the slanted face from (4, 3) to (1, 7). The
groove between them, outside the solid, opens at the angle θ with cos θ = −3/5, about 126.87°.

Chamfer that edge by its throat 1/2, as a weld is sized: a concave chamfer adds material, filling
the groove. In the plane square to the edge the chamfer's section is an isosceles triangle: its two
legs, along the two faces from the edge, are equal, d = (1/2)/cos(θ/2) = √5/2 ≈ 1.118, and its third
side, the chamfer's face, lies 1/2 from the edge. The chamfer is the plane through the lines
(4 + d, y, 3) and (4 − 3d/5, y, 3 + 4d/5), and it runs the whole length of the edge.

**As a set:**

$$\{\,(x, z) \in P \cup T,\ 0 \le y \le 8\,\}$$

where P is the polygon above and T the triangle with corners (4, 3), (4 + d, 3) and (4 − 3d/5, 3 + 4d/5).
