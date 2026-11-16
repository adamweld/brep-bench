+++
title = "A T-block with its sides drafted until they close over its bar and its stem"
tags = ["draft", "planar", "concave", "ridge", "closing", "analytic"]

[volume]
value = 182.0
exact = "∫₀² (152 − 72d + 4d²) dd + ∫₂³ (6 − 2d)(16 − 2d) dd = 512/3 + 34/3 (tan 45° = 1, so d = z)"
tex = '\int_0^2\left(152-72d+4d^2\right)\mathrm{d}d + \int_2^3(6-2d)(16-2d)\,\mathrm{d}d = \frac{512}{3}+\frac{34}{3}'

[area]
value = 366.9604614807104
exact = "A₀ + A₀/sin 45° = 152(1 + √2), A₀ = 152 (the floor, and the sides, which lie over the whole floor)"
tex = 'A_0 + \frac{A_0}{\sin 45^\circ} = 152\left(1+\sqrt{2}\right),\quad A_0 = 152'
+++

A T-shaped block: the outline through (0, 0), (20, 0), (20, 4), (13, 4), (13, 16), (7, 16), (7, 4)
and (0, 4) in the plane z = 0, extruded from z = 0 to z = 8. Its bar is 20 by 4 and its stem, 6
wide, stands 12 past the bar.

Draft all eight sides by 45°, with the pull direction +z and the neutral plane z = 0, the plane of
its floor: each side turns about its bottom edge until it leans 45° from the z direction, inward as
it rises. The sides are planes, and the solid is what lies under all of them above the floor. The
bar's two long sides meet in a ridge at height 2 and the stem's at height 3, both under the block's
top at z = 8, so nothing of the top is left; the two sides in the line y = 4 lie in one plane.

Its section at height z = d is the T moved d into its material, each side parallel to itself: for
d < 2 the bar 20 − 2d by 4 − 2d with the stem 6 − 2d wide on it, reaching y = 16 − d; for
2 ≤ d < 3 the stem alone, the rectangle from (7 + d, d) to (13 − d, 16 − d); nothing above.

**As a set:**

$$\bigl\{\,(x,y) \in \bigl([d,\,20-d]\times[d,\,4-d]\bigr) \cup
\bigl([7+d,\,13-d]\times[d,\,16-d]\bigr),\ 0 \le d < 3\,\bigr\}, \quad d = z$$
