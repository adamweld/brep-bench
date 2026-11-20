+++
title = "A block with a square boss and a round boss stacked on it"
tags = ["feature", "boss", "union", "planar", "cylinder", "analytic"]
origin = "OCCT feat/featprism/G5, OCCT feat/featprism/H5"

[volume]
value = 18871.238898038469
exact = "40·40·10 + 20·20·6 + π·5²·6 = 18400 + 150π"
tex = '40\cdot 40\cdot 10 + 20\cdot 20\cdot 6 + \pi\cdot 5^2\cdot 6'

[area]
value = 5468.4955592153876
exact = "block 2·1600 + 4·40·10 = 4800 (the square boss's footprint on its top is replaced by the boss's own top), square boss's sides 4·20·6 = 480, round boss's side 2π·5·6 = 60π (its top replaces the disc it stands on)"
tex = '\left(2\cdot 40^2 + 4\cdot 40\cdot 10\right) + 4\cdot 20\cdot 6 + 2\pi\cdot 5\cdot 6'
+++

A square block with a square boss on its top, and a round boss on that, joined into one solid.

- The block: 0 ≤ x ≤ 40, 0 ≤ y ≤ 40, 0 ≤ z ≤ 10.
- The square boss: 6 ≤ x ≤ 26, 6 ≤ y ≤ 26, from z = 10 to z = 16, standing on the block's top.
- The round boss: a cylinder of radius 5 on the axis x = 16, y = 16, from z = 16 to z = 22, standing
  on the square boss's top, wholly inside its outline.

**As a set:**

$$\bigl([0,40]^2\times[0,10]\bigr) \cup \bigl([6,26]^2\times[10,16]\bigr) \cup \{\,(x-16)^2 + (y-16)^2 \le 5^2,\ 16 \le z \le 22\,\}$$
