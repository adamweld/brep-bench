+++
title = "A round rib crossing a leaning block's rounded top edge"
tags = ["feature", "rib", "linear", "union", "cylinder", "planar", "draft", "fillet", "analytic"]
origin = "OCCT tests/feat/featlf/C1, OCCT tests/feat/featlf/C2, OCCT tests/feat/featlf/C3, OCCT tests/feat/featlf/C4"

[volume]
value = 29458.42189841481
exact = "30·A, A the area of the section: the quadrilateral less the corner the round takes off, joined to the circular segment of the disc of radius 8 about (28, 26) above z = 22 (a region bounded by lines and circular arcs; A = 981.9473966138, evaluated from its boundary arcs at 12 digits)"

[area]
value = 6285.775591519776
exact = "2A + 30·P, P the perimeter of that section: the top from x = 0 to 28 − √60, the rib's arc from there round to (28 + √48, 22), its underside back to the round at (26 + √20, 22), the round's arc from there down to the leaning side's tangent point, the leaning side to (40, 0), the bottom and the back (P = 144.0626932764; evaluated from the boundary arcs at 12 digits)"
+++

A block whose side leans, with its top edge rounded. Its section in the xz plane is the quadrilateral
with corners (x, z) = (0, 0), (40, 0), (30, 24) and (0, 24), swept along y from y = 0 to y = 30. The
side from (40, 0) to (30, 24) is the plane 12x + 5z = 480, which leans in as it rises. The edge
between that side and the top, at (30, 24), is rounded to radius 6: a cylinder
about the line x = 26, z = 18 (parallel to y), touching the top at x = 26 and the leaning side at
(26 + 72/13, 18 + 30/13) ≈ (31.54, 20.31).

A round rib lies along the block's top and goes down over the rounded edge. Its section, in planes
perpendicular to y, is the part of the disc of radius 8 about (x, z) = (28, 26) that lies above the
line z = 22 (a little more than the upper half of the disc), swept along y over the block's whole
length, from y = 0 to y = 30, so its two ends lie flush with the block's. It covers the top face from
x = 28 − √60 ≈ 20.25 on, and the rounded edge down to z = 22, where its flat underside shows
beyond the round, from x = 26 + √20 ≈ 30.47 on the round to x = 28 + √48 ≈ 34.93. Below z = 22 the
rounded edge and the leaning side are left as they were. The block and the rib are one solid.

**As a set:**

$$\Bigl(\{\,0 \le x,\ 12x + 5z \le 480,\ 0 \le z \le 24,\ 0 \le y \le 30\,\} \setminus \{\,x > 26,\ 5(x - 26) < 12(z - 18),\ (x - 26)^2 + (z - 18)^2 > 36\,\}\Bigr) \cup \{\,0 \le y \le 30,\ (x - 28)^2 + (z - 26)^2 \le 64,\ z \ge 22\,\}$$
