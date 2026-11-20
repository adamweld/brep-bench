+++
title = "Five rings placed apart, fused"
tags = ["boolean", "union", "torus", "analytic", "lumps"]
origin = "OCCT boolean/gdml_public/B6"

[volume]
value = 394.78417604357435
exact = "5·2π²Rr² = 5·8π²: five tori of major radius 4 and tube radius 1; nothing joins them"
tex = '5\cdot 2\pi^2 R r^2 = 40\pi^2'

[area]
value = 789.5683520871487
exact = "5·4π²Rr = 5·16π²"
tex = '5\cdot 4\pi^2 R r = 80\pi^2'
+++

Five equal rings, fused one after another into a single shape. Each is a torus about a vertical axis, with major radius 4 (the radius of the circle through the tube's center) and tube radius 1, lying flat with its center on the x axis at x = 0, 12, 24, 36 and 48. Neighboring rings are far apart (the outer radius is 5, the centers 12 apart), so the fuse joins nothing: the result is five separate solid rings.

**As a set:**

$$\bigcup_{k=0}^{4} \{\,(\sqrt{(x - 12k)^2 + y^2} - 4)^2 + z^2 \le 1\,\}$$
