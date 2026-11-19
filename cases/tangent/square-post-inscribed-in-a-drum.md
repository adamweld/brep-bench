+++
title = "A square post inscribed in a drum, its edges on the drum's wall"
tags = ["tangent", "boolean", "union", "analytic", "cylinder", "planar", "coincident"]
origin = "OCCT tests/bugs/moddata_2/bug259"

[volume]
value = 1085.3981633974483
exact = "πR²·10 + s²·6 = 250π + 300, R = 5, s = 5√2 the square's side"
tex = '10\pi R^2 + 6 s^2 = 250\pi + 300,\quad R = 5,\ s = 5\sqrt2'

[area]
value = 640.9445255232404
exact = "2πR·10 + πR² + (πR² − s²) + 4s·6 + s² = 100π + 25π + 25π − 50 + 120√2 + 50 = 150π + 120√2: the drum's wall, bottom and top outside the square, and the post's four sides and top above it"
tex = '20\pi R + \pi R^2 + \left(\pi R^2 - s^2\right) + 24 s + s^2 = 150\pi + 120\sqrt2'
+++

A cylinder (a drum) of radius 5 on the z axis, from z = 0 to z = 10, united with a square prism (a
post) from z = 0 to z = 16 on the square inscribed in the drum's circle: its corners are on the
circle of radius 5 at φ = 30°, 120°, 210° and 300° (φ the angle about the z axis from +x toward
+y), and its side is 5√2.

Up to z = 10 the post is inside the drum, and its four upright edges lie on the drum's wall, each
along a line of the cylinder. Above, it stands 6 out of the drum's top. The union is the drum with
the post standing out of it. The drum's wall is whole: the post's edges are not edges of the
result below z = 10. The drum's top is left as the four pieces of its disk outside the square, each
meeting the next only at a corner of the square, where an edge of the post rises from the drum's
rim.

If your drum's cylinder has a seam, put it at φ = 0, between two of the post's edges.

**As a set:**

$$\{\,\rho \le 5,\ 0 \le z \le 10\,\} \cup R_z(30^\circ)\,\{\,|x| + |y| \le 5,\ 0 \le z \le 16\,\}$$
