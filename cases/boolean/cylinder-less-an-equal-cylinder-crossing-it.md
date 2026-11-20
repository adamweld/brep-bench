+++
title = "A cylinder less an equal cylinder crossing it"
tags = ["boolean", "cut", "analytic", "singular", "pieces"]
origin = "OCCT tests/boolean/bopcut_simple/ZD8"

[volume]
value = 138.7433388230814
exact = "π·3²·10 less the Steinmetz solid 16r³/3 = 144 of the two cylinders' common part"
tex = '\pi r^2 \cdot 10 - \tfrac{16}{3} r^3,\quad r = 3'

[area]
value = 245.04422698000388
exact = "the first cylinder's own 2π·3·10 + 2π·3²; the wall it loses inside the second and the wall of the second it gains inside the first are each 8r² = 72 and cancel"
tex = '2\pi r\cdot 10 + 2\pi r^2 + 8r^2 - 8r^2 = 78\pi'
+++

Two cylinders of radius 3 and length 10, crossing at right angles at their midpoints: the first on the x axis from x = −5 to x = 5, the second on the y axis from y = −5 to y = 5. Cut the second away from the first.

The cylinders have the same radius, so their intersection is two ellipses, in the planes x = y and x = −y, which cross at the two points (0, 0, ±3), where the walls touch. What is left is two separate pieces, one in x > 0 and one in x < 0: the two ends of the first cylinder, each with a concave face where the second cylinder's wall was.

**As a set:**

$$\{y^2 + z^2 \le 9,\ |x| \le 5\} \setminus \{x^2 + z^2 \le 9,\ |y| \le 5\}$$
