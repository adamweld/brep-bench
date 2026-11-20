+++
title = "An elliptic cone frustum shelled with its narrow top open"
tags = ["shell", "inward", "loft", "cone", "elliptic"]
origin = "OCCT tests/bugs/modalg_2/bug5805_34"
settled = false
+++

A frustum with elliptic ends on the z axis, made by a ruled loft: its side ruled between the ellipse
x²/36 + y²/16 = 1 at z = 0 and the ellipse x²/9 + y²/4 = 1 at z = 8 (the second is the first
scaled by a half about the axis), and closed by the two elliptic disks they bound. The ruled
surface between them is the elliptic cone through them, so the frustum is
(x / (6 − 3z/8))² + (y / (4 − z/4))² ≤ 1, 0 ≤ z ≤ 8.

Shell it inward to a wall 1 thick, open at its narrow top face, z = 8. The cavity is every point
of the frustum at least 1 from its side and at least 1 above its bottom, running up to the top
plane, which is the opening; the wall is everything else in the frustum, its rim at the top the
ring between the frustum's ellipse and the cavity's. The result is the frustum less that cavity:
a cup whose floor is the plane z = 1.

The inward offset of an elliptic cone is not a cone of any kind, so the cavity's wall has no closed
form and no volume or area is stated.

**As a set:**

$$F \setminus \{\, p \in F : d(p,\ \partial F \cap \{z < 8\}) \ge 1 \,\}, \quad
F = \{\, (x/(6 - 3z/8))^2 + (y/(4 - z/4))^2 \le 1,\ 0 \le z \le 8 \,\}$$
