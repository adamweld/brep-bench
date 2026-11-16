+++
title = "An L-block with its sides drafted until they close over its arms"
tags = ["draft", "planar", "concave", "ridge", "closing", "analytic"]

[volume]
value = 306.7173305069887
exact = "(1/t)·∫₀^2.5 (5 − 2d)(30 − 2d) dd = 2125√3/12, t = tan 30°"
tex = '\frac{1}{t}\int_0^{5/2}(5-2d)(30-2d)\,\mathrm{d}d = \frac{2125\sqrt{3}}{12},\quad t = \tan 30^\circ'

[area]
value = 450.0
exact = "A₀ + A₀/sin 30°, A₀ = 150 (the floor, and the sides, which lean 30° from the z direction and lie over the whole floor)"
tex = 'A_0 + \frac{A_0}{\sin 30^\circ},\quad A_0 = 150'
+++

An L-shaped block: the outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15) in
the plane z = 0, extruded from z = 0 to z = 8. Both of its arms are 5 wide.

Draft all six sides by 30°, with the pull direction +z and the neutral plane z = 0, the plane of
its floor: each side turns about its bottom edge until it leans 30° from the z direction, inward as
it rises. The sides are planes, and the solid is what lies under all of them above the floor: the
sides of each arm meet in a ridge over its middle, at the height 2.5 / tan 30° ≈ 4.33, under the
block's top at z = 8, so nothing of the top is left.

The result has seven faces: the floor and the six sides. Its section at height z is the L moved
d = z·tan 30° into its material, each side parallel to itself, the outline through (d, d),
(20 − d, d), (20 − d, 5 − d), (5 − d, 5 − d), (5 − d, 15 − d) and (d, 15 − d), for d < 2.5, and
nothing above.

**As a set:**

$$\bigl\{\,(x,y) \in
\operatorname{poly}\bigl((d,d),(20-d,d),(20-d,5-d),(5-d,5-d),(5-d,15-d),(d,15-d)\bigr),\ 0 \le d <
\tfrac52\,\bigr\}, \quad d = z\tan 30^\circ$$
