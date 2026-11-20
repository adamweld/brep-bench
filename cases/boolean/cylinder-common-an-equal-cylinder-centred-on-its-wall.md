+++
title = "A cylinder and an equal cylinder centred on its wall, their common part"
tags = ["boolean", "common", "analytic", "cylinder"]
origin = "OCCT tests/boolean/bopcommon_simple/ZF2"

[volume]
value = 66.33196372487286
exact = "6A, A = r²(2π/3 − √3/2) with r = 3: the lens of two circles of radius r whose centres are r apart"
tex = '6A,\quad A = r^2\bigl(\tfrac{2\pi}{3} - \tfrac{\sqrt3}{2}\bigr),\ r = 3'

[area]
value = 97.50887826111266
exact = "2A + 6·4π: two flat lens ends, and two wall arcs of 2πr/3 each, 4πr/3 = 4π together, 6 high"
tex = '2A + 6\cdot 4\pi'
+++

Two cylinders of radius 3 and height 6 standing on the xy plane, from z = 0 to z = 6: one with its axis the z axis, the other with its axis the vertical line through (3, 0). Each one's axis lies on the other's wall, and the two have the same ends. Take the part common to the two.

What is left is a lens: the region where two circles of radius 3 with centres 3 apart overlap, from z = 0 to z = 6. It has two flat ends and two wall faces, one from each cylinder, meeting along two vertical edges.

**As a set:**

$$\{\,\rho \le 3,\ 0 \le z \le 6\,\} \cap \{\,(x-3)^2 + y^2 \le 3^2,\ 0 \le z \le 6\,\}$$
