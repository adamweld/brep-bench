+++
title = "Three equal cylinders crossing at right angles through one point, their common part"
tags = ["boolean", "common", "analytic", "cylinder", "ssi"]
origin = "OCCT bugs/modalg_7/bug31835_1, OCCT bugs/modalg_7/bug31835_2"

[volume]
value = 585.7864376269049
exact = "8(2 − √2)r³, r = 5"
tex = '8\left(2 - \sqrt{2}\right)r^3,\quad r = 5'

[area]
value = 351.471862576143
exact = "3 · 8(2 − √2)r²: a cylinder's wall bounds the part where it lies inside the other two, |z| ≤ r·min(|sin φ|, |cos φ|) at the angle φ round it, an area r·∫2r·min(|sin φ|, |cos φ|)dφ = 8(2 − √2)r², r = 5"
tex = '3\cdot 8\left(2 - \sqrt{2}\right)r^2,\quad r = 5'
+++

Three cylinders of radius 5 and length 20. The first lies along the x axis, from x = −10 to 10; the second along the y axis, from y = −10 to 10; the third along the z axis, from z = −10 to 10. Their axes are mutually perpendicular and meet at the origin. Make the solid common to all three.

The result is bounded by three bands of cylinder wall, one from each cylinder, each the part of its wall that lies inside the other two. Each pair of walls meets in two ellipses, in the planes x = ±y, y = ±z and x = ±z, and the solid's edges are arcs of them. Its eight corners, each where all three walls meet, are the points $(\pm 5/\sqrt{2},\ \pm 5/\sqrt{2},\ \pm 5/\sqrt{2})$.

**As a set:**

$$\{\,y^2 + z^2 \le 5^2,\ |x| \le 10\,\} \cap \{\,x^2 + z^2 \le 5^2,\ |y| \le 10\,\} \cap \{\,x^2 + y^2 \le 5^2,\ |z| \le 10\,\}$$
