+++
title = "A cap rib wrapped over a block's leaning top edge"
tags = ["feature", "rib", "linear", "union", "planar", "draft", "analytic"]
origin = "OCCT tests/feat/featlf/B4, OCCT tests/feat/featlf/C5"

[volume]
value = 23780.0
exact = "the block, 22800, plus the cap, 14 times the area of its quadrilateral (shoelace: 70) = 22800 + 14·70 = 23780"

[area]
value = 5381.0002570070965
exact = "the block's 1200 + 1080 + 1520 + 600 + 600√1.04, less the part of the top face the cap covers, 8·14, and of the leaning side, 14·√104 (the segment from (38, 10) to (36, 20)), plus the cap's two sloping faces 14(√260 + √340) and its two ends 2·(the quadrilateral's area)"
+++

A block whose side leans: everything with 0 ≤ y ≤ 30, 0 ≤ z ≤ 20 and 0 ≤ x ≤ 40 − z/5. Its side
x = 40 − z/5 leans in as it rises, from x = 40 at the bottom to x = 36 at the top; its top face is
the rectangle 0 ≤ x ≤ 36, 0 ≤ y ≤ 30 at z = 20, and the edge between the top and the side is the
line x = 36, z = 20.

A cap rib wraps over that edge. Its profile, in planes perpendicular to y, is the open two-segment
line from (x, z) = (28, 20) on the top face out to (42, 28) and back in to (38, 10) on the leaning
side; the rib fills the space between that line and the block. In the xz plane it is the
quadrilateral with corners (28, 20), (42, 28), (38, 10) and (36, 20), which goes round the edge
(the corner at (36, 20) is the block's own edge, where the rib's angle is 270°). It is swept along y
from y = 8 to y = 22, so its two ends are flat, in the planes y = 8 and y = 22. The block and the
rib are one solid.

**As a set:**

$$\{\,0 \le x \le 40 - \tfrac{z}{5},\ 0 \le y \le 30,\ 0 \le z \le 20\,\} \cup \{\,8 \le y \le 22,\ (x, z) \in \mathrm{poly}\bigl((28, 20), (42, 28), (38, 10), (36, 20)\bigr)\,\}$$
