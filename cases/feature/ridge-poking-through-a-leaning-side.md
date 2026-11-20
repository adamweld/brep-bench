+++
title = "A ridge poking through a block's leaning side"
tags = ["feature", "rib", "linear", "union", "planar", "draft", "analytic"]
origin = "OCCT tests/feat/featlf/A3, OCCT tests/feat/featlf/B3, OCCT tests/feat/featlf/B6, OCCT tests/feat/featlf/C7, OCCT tests/feat/featlf/C8, OCCT tests/feat/featlf/D2"

[volume]
value = 23881.6
exact = "the block, 22800, plus the ridge outside it: ∫₁₈²⁰ 1.5(26 − z)(4 + z/5) dz (below the top, where only the ridge's part from the leaning side x = 40 − z/5 to 44 is outside the block) + 34·∫₂₀²⁶ 1.5(26 − z) dz (above the top, the whole ridge, 34 long) = 22800 + 1081.6"

[area]
value = 5471.666459674045
exact = "the block's 1200 + 1080 + 1520 + 600 + 600√1.04, less the top face the ridge covers, 26·9 = 234, and the strip of the leaning side it covers, √1.04·∫₁₈²⁰ 1.5(26 − z) dz, plus the ridge's two sloping faces, 2·1.25·(∫₁₈²⁰ (4 + z/5) dz + 34·6), its two ends (27 above the top at x = 10, 48 whole at x = 44), and its underside 12·7.6"
+++

A block whose side leans: everything with 0 ≤ y ≤ 30, 0 ≤ z ≤ 20 and 0 ≤ x ≤ 40 − z/5. Its side
x = 40 − z/5 leans in as it rises, from x = 40 at the bottom to x = 36 at the top; its top face is
the rectangle 0 ≤ x ≤ 36, 0 ≤ y ≤ 30 at z = 20.

A ridge runs along the block's top over the leaning side. Its section, in planes perpendicular to x,
is the triangle with corners (y, z) = (9, 18), (21, 18) and (15, 26): a roof 12 wide and 8 high, whose
foot lies 2 below the top face. It is swept along x from x = 10 to x = 44, so its two ends are flat,
in the planes x = 10 and x = 44. The ridge's two sloping faces run over the top face and past the
leaning side, which they cross between z = 18 and z = 20; beyond the side the ridge's flat underside
at z = 18 shows, from x = 36.4 to 44. The block and the ridge are one solid.

**As a set:**

$$\{\,0 \le x \le 40 - \tfrac{z}{5},\ 0 \le y \le 30,\ 0 \le z \le 20\,\} \cup \{\,10 \le x \le 44,\ 18 \le z \le 26,\ |y - 15| \le \tfrac34 (26 - z)\,\}$$
