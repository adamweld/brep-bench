+++
title = "A cylinder united with a frustum hidden inside it, as tall as it is"
tags = ["boolean", "union", "analytic", "cylinder", "cone", "coincident"]
origin = "OCCT tests/boolean/bopfuse_simple/ZJ6"

[volume]
value = 471.23889803846896
exact = "150π: the union is the cylinder itself"
tex = '150\pi'

[area]
value = 345.57519189487726
exact = "50π + 60π: the cylinder's two ends and its wall"
tex = '50\pi + 60\pi'
+++

A cylinder of radius 5, its axis the z axis, from z = 0 to z = 6.

A frustum of a cone, coaxial with it and as tall as it: from z = 0 to z = 6, radius 3 at z = 0, narrowing in a straight line to radius 2 at z = 6. Make the union of the two.

The frustum's base lies in the cylinder's bottom face and its top in the cylinder's top face, both wholly inside their outlines; its cone surface is clear of the cylinder's wall. Everything of the frustum lies inside the cylinder or on its surface, so the union is the cylinder itself.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 6\,\} \cup \{\,0 \le z \le 6,\ \rho \le 3 - \tfrac16 z\,\}$$
