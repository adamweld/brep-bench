+++
title = "A triangular rib crossing a leaning block's rounded top edge"
tags = ["feature", "rib", "linear", "union", "cylinder", "planar", "draft", "fillet", "analytic"]
origin = "OCCT tests/feat/featlf/D3, OCCT tests/feat/featlf/D4"

[volume]
value = 25532.85587142794
exact = "30·A, A the area of the section: the quadrilateral less the corner the round takes off, joined to the triangle (a region bounded by lines and one circular arc; A = 851.0951957143, evaluated from its boundary at 12 digits)"

[area]
value = 5606.223627330099
exact = "2A + 30·P, P the perimeter of that section (P = 130.1344411972; evaluated from the boundary at 12 digits)"
+++

A block whose side leans, with its top edge rounded. Its section in the xz plane is the quadrilateral
with corners (x, z) = (0, 0), (40, 0), (30, 24) and (0, 24), swept along y from y = 0 to y = 30. The
side from (40, 0) to (30, 24) is the plane 12x + 5z = 480, which leans in as it rises. The edge
between that side and the top, at (30, 24), is rounded to radius 6: a cylinder
about the line x = 26, z = 18 (parallel to y), touching the top at x = 26 and the leaning side at
(26 + 72/13, 18 + 30/13) ≈ (31.54, 20.31).

A triangular rib lies against the leaning side, across the start of the round. Its section, in
planes perpendicular to y, is the triangle with corners (x, z) = (28, 18), (38, 18) and (33, 22),
swept along y over the block's whole length, from y = 0 to y = 30. It comes out of the leaning side
at z = 18 (where the side is at x = 32.5) with its flat underside, and its two sloping faces meet
the round: the left one, from (28, 18) to (33, 22), crosses the leaning side and then the round
near the point where they touch. The rounded edge above the rib and the leaning side below it are
left as they were. The block and the rib are one solid.

**As a set:**

$$\Bigl(\{\,0 \le x,\ 12x + 5z \le 480,\ 0 \le z \le 24,\ 0 \le y \le 30\,\} \setminus \{\,x > 26,\ 5(x - 26) < 12(z - 18),\ (x - 26)^2 + (z - 18)^2 > 36\,\}\Bigr) \cup \{\,0 \le y \le 30,\ (x, z) \in \mathrm{poly}\bigl((28, 18), (38, 18), (33, 22)\bigr)\,\}$$
