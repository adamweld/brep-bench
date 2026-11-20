+++
title = "Two bars crossed at an angle, united"
tags = ["boolean", "union", "planar", "analytic"]
origin = "OCCT boolean/gdml_public/B1, boolean/gdml_public/C4"

[volume]
value = 76.0
exact = "2·(24 + 24 − 10): both bars have the area of their section, 12 × 2 and 8 × 3, and the two overlap in a parallelogram of area 10"
tex = '2\,(24 + 24 - 10) = 76'

[area]
value = 142.66666666666666
exact = "the top and bottom 2·38 and the outline of the section 2·(100/3): the union of the two outlines, each less the part inside the other"
tex = '2\cdot 38 + 2\cdot\tfrac{100}{3} = \tfrac{428}{3}'
+++

Two bars lying across each other, united into one solid. Both are 2 high, standing on the plane z = 0. The first is the box from (−6, −1, 0) to (6, 1, 2). The second is 8 long and 3 wide: the box from (−4, −1.5, 0) to (4, 1.5, 2), turned about the z axis through the origin by the angle whose tangent is 3/4 (cosine 4/5, sine 3/5), right-handed, from +x toward +y.

Their top faces are in one plane and so are their bottoms. The second bar's ends and its sides stick out of the first (and the first's out of the second), and they overlap in a parallelogram with corners (−23/6, −1), (7/6, −1), (23/6, 1) and (−7/6, 1).

**As a set**, with u = (4x + 3y)/5 and v = (−3x + 4y)/5:

$$\bigl(\{\,|x| \le 6,\ |y| \le 1\,\} \cup \{\,|u| \le 4,\ |v| \le 1.5\,\}\bigr) \times [0, 2]$$
