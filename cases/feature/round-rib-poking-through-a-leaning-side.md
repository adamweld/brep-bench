+++
title = "A round rib poking through a block's leaning side"
tags = ["feature", "rib", "linear", "union", "cylinder", "planar", "draft", "analytic"]
origin = "OCCT tests/feat/featlf/A1, OCCT tests/feat/featlf/A2, OCCT tests/feat/featlf/C1, OCCT tests/feat/featlf/C2, OCCT tests/feat/featlf/C3, OCCT tests/feat/featlf/C4"

[volume]
value = 24095.552397262018
exact = "the block, 30·∫₀²⁰ (40 − z/5) dz = 22800, plus 12·A where A is the area of the rib's section outside the block's: A = 32π (the half disc above z = 20) + ∫_{−2}^{0} 2√(64 − t²) dt (the strip of the disc from z = 18 to 20) − ∫₁₈²⁰ (8 − z/5 + √(64 − (z − 20)²)) dz (the part of that strip inside the block's section)"

[area]
value = 5425.334247056796
exact = "the block's 1200 + 1080 + 1520 + 600 + 600√1.04 (bottom, top, two ends, back, leaning side) less the top face the rib covers, 12·12, and the strip of the leaning side it covers, 12·2√1.04, plus the rib's round face 12·8·(π + asin(1/4)), its underside 12(32 + √60 − 36.4), and its two ends 2A"
+++

A block whose side leans: everything with 0 ≤ y ≤ 30, 0 ≤ z ≤ 20 and 0 ≤ x ≤ 40 − z/5. Its side
x = 40 − z/5 leans in as it rises, from x = 40 at the bottom to x = 36 at the top; its top face is
the rectangle 0 ≤ x ≤ 36, 0 ≤ y ≤ 30 at z = 20.

A round rib runs along the block's top near that side. Its section, in planes perpendicular to y, is
the part of the disc of radius 8 about (x, z) = (32, 20) that lies above the line z = 18: a little
more than the upper half of the disc. It is swept along y from y = 9 to y = 21, so its two ends are
flat, in the planes y = 9 and y = 21. The rib sticks out above the top face, and its lower part
crosses the leaning side: between z = 18 and z = 20 the side lies inside the rib, and the rib's flat
underside at z = 18 shows beyond it, from x = 36.4 to 32 + √60 ≈ 39.75. The block and the rib are
one solid.

**As a set:**

$$\{\,0 \le x \le 40 - \tfrac{z}{5},\ 0 \le y \le 30,\ 0 \le z \le 20\,\} \cup \{\,9 \le y \le 21,\ (x - 32)^2 + (z - 20)^2 \le 64,\ z \ge 18\,\}$$
