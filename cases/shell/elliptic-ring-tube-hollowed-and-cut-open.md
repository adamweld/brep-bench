+++
title = "A tube round an ellipse, hollowed and cut open by a plane"
tags = ["shell", "sweep", "ellipse", "oblique"]
origin = "OCCT tests/bugs/modalg_7/bug25395_2"
settled = false
+++

The path is the ellipse in the xy plane about the origin with semi-axis 20 along x and 12 along y.
Sweep a circle of radius 3 along it, the circle always square to the path, once all the way round:
a solid ring-shaped tube. Sweep a circle of radius 2 along the same ellipse in the same way: its
core. The tube's wall is the first less the second, a hollow ring with a wall 1 thick all round
its section.

Cut the wall by the half-space x ≤ 15, taking away the part where x > 15, which lies at the
ellipse's right-hand end. The plane x = 15 meets the ring in two sections, one above the x axis
and one below it, each a slanted, not square, cut across the wall: two flat openings, through
which the hollow shows.

The result is the wall, one curved piece open at those two openings, thickness 1 inward from the
tube's outside everywhere: the swept tube less its core, cut by the plane.

The slanted sections and the swept surfaces have no closed form for the volume and the area, so
the case is unsettled until two kernels agree.

**As a set:**

$$\bigl(\{\,d(p, E) \le 3\,\} \setminus \{\,d(p, E) < 2\,\}\bigr) \cap \{\,x \le 15\,\}$$

where E is the ellipse $x^2/400 + y^2/144 = 1,\ z = 0$ and d the distance from it.
