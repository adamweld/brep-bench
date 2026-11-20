+++
title = "Two equal tori united, the second centered on the first's core circle"
tags = ["boolean", "union", "torus", "intersection-curve"]
origin = "OCCT tests/bugs/modalg_7/bug29910_1"
settled = false
+++

Two tori with their axes parallel to the z axis, each with major radius 10 (the circle through the
middle of the tube) and minor radius 3 (the tube). The first is centered at the origin; the second at
(10, 0, 0), so its center lies on the first's core circle and its core circle passes through the
first's center. The tubes overlap round the two points (5, ±5√3, 0) where the core circles cross, and
their surfaces meet in closed curves that cross themselves.

Unite them.

**As a set:**

$$T_1 \cup T_2,\quad T_1 = \{\,(\rho - 10)^2 + z^2 \le 9\,\},\quad T_2 = \{\,(\sqrt{(x-10)^2 + y^2} - 10)^2 + z^2 \le 9\,\}$$
