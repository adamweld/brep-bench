+++
title = "A block with a rounded corner bridging a stepped block's concave corner and leaning side"
tags = ["feature", "rib", "linear", "union", "cylinder", "planar", "draft", "fillet", "analytic"]
origin = "OCCT tests/feat/featlf/C9, OCCT tests/feat/featlf/D6"

[volume]
value = 22138.840043236458
exact = "30·A, A the area of the section: the hexagon less the corner the round takes off, joined to the rectangle with its rounded corner (a region bounded by lines and circular arcs; A = 737.9613347745, evaluated from its boundary arcs at 12 digits)"

[area]
value = 5077.64651081449
exact = "2A + 30·P, P the perimeter of that section: the bottom, the leaning side up to z = 14.4 then the rib's right face x = 34 up to z = 22, the rib's top back to the round at x = 26 + √20, the round's arc to the top at x = 26, the top to the step, the step down to z = 22, the rib's top from x = 14 to 12, its quarter-circle arc to (8, 18), its left face down to z = 12, the ledge back to x = 0 and the back (P = 120.0574613755; evaluated from the boundary arcs at 12 digits)"
+++

A stepped block whose side leans, with its top edge rounded. Its section in the xz plane is the
hexagon with corners (x, z) = (0, 0), (40, 0), (30, 24), (14, 24), (14, 12) and (0, 12), swept along y
from y = 0 to y = 30: a low ledge, z = 12 for x from 0 to 14, then a step up at x = 14 to the top,
z = 24. The step makes a concave corner at (14, 12). The side from (40, 0) to (30, 24) is the plane
12x + 5z = 480, which leans in as it rises. The edge between that side and the top, at (30, 24), is
rounded to radius 6: a cylinder about the line x = 26, z = 18 (parallel to y), touching the top at
x = 26 and the leaning side at (26 + 72/13, 18 + 30/13) ≈ (31.54, 20.31).

A block bridges the step and the leaning side. Its section is the rectangle 8 ≤ x ≤ 34,
10 ≤ z ≤ 22 with its corner at (8, 22) rounded by a quarter circle of radius 4 about (12, 18), swept
along y over the whole length, from y = 0 to y = 30. It fills the space over the ledge from x = 8 to
the step, up to z = 22, covering the concave corner (14, 12); and on the other side its right face
x = 34 comes out of the leaning side at z = 14.4 and runs up to z = 22, beyond the round, where its
top shows from x = 26 + √20 ≈ 30.47 to x = 34. The step above z = 22 and the rounded edge above
that are left as they were. The stepped block and the rib are one solid.

**As a set:**

$$\Bigl(\{\,0 \le x,\ 12x + 5z \le 480,\ 0 \le z \le 24,\ 0 \le y \le 30\,\} \setminus \{\,x < 14,\ z > 12\,\} \setminus \{\,x > 26,\ 5(x - 26) < 12(z - 18),\ (x - 26)^2 + (z - 18)^2 > 36\,\}\Bigr) \cup \{\,0 \le y \le 30,\ (x, z) \in R\,\},\quad R = [8, 34] \times [10, 22] \setminus \{\,x < 12,\ z > 18,\ (x - 12)^2 + (z - 18)^2 > 16\,\}$$
