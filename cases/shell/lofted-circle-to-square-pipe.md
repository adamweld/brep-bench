+++
title = "A pipe lofted from a circle to a square, open at both ends"
tags = ["shell", "inward", "loft", "ruled", "freeform"]
origin = "OCCT tests/offset/bugs/bug5806"
settled = false
+++

A solid lofted between two sections along the z axis: the circle of radius 3 about the axis in the
plane z = 0, and the boundary of the square |x| ≤ 4, |y| ≤ 4 in the plane z = 10. The loft is ruled:
each point of the circle, (3 cos θ, 3 sin θ, 0), is joined by a straight line to the point of the
square's boundary in the same direction from the axis, (4 cos θ, 4 sin θ, 10)/max(|cos θ|, |sin θ|),
for every angle θ. So its side is made of the lines so drawn, four ruled faces that meet in four
sharp edges, the lines from the circle's points at 45°, 135°, 225° and 315° to the square's corners.
Its ends are the flat disk in z = 0 and the flat square in z = 10.

Shell it inward to a wall 1/2 thick, open at both ends, a pipe. The outside of the wall is the loft's
side; the inside is the set of points of the loft at least 1/2 from that side, which runs on from
z = 0 to z = 10 to meet the planes of the openings, so the rims are flat in those planes. The side
is not developable and the inner surface is its parallel surface, with no closed form, so the case
is unsettled until two kernels agree.

**As a set:**

$$S \setminus \bigl(S \ominus_{\text{side}} B_{1/2}\bigr),\quad S = \{\, \rho \le (1 - \tfrac{z}{10})\,3 + \tfrac{z}{10}\cdot\tfrac{4}{\max(|\cos\varphi|,\,|\sin\varphi|)},\ 0 \le z \le 10 \,\}$$

where the second set is the points of S at least 1/2 from S's side, the ends not counted.
