+++
title = "A cube edge rounded with a radius that falls to nothing at its middle and grows again"
tags = ["fillet", "variable-radius", "degenerate", "analytic"]
origin = "OCCT tests/blend/tolblend_buildvol/A4"

[volume]
value = 998.28318530717959
exact = "1000 − (1 − π/4)∫₀¹⁰ r² dz, r = 2(1 − z/5)²: ∫r² dz = 8, so 1000 − 8(1 − π/4)"
tex = '1000 - \left(1 - \tfrac{\pi}{4}\right)\int_0^{10} r^2\,dz = 1000 - 8\left(1 - \tfrac{\pi}{4}\right),\quad r = 2\left(1 - \tfrac{z}{5}\right)^2'

[area]
value = 595.60046121064608
exact = "600 − 2∫₀¹⁰ r dz − (1 − π/4)(r(0)² + r(10)²) + J = 600 − 40/3 − 8(1 − π/4) + J, J = ∫₀¹⁰∫₀^{π/2} r√(1 + r′²(1 − sin θ − cos θ)²) dθ dz = 10.650609236799832, r′ = −(4/5)(1 − z/5) (the round’s area element is r√(1 + r′²(1 − sin θ − cos θ)²) dθ dz)"
tex = '\begin{array}{l} 600 - \tfrac{40}{3} - 8\left(1 - \tfrac{\pi}{4}\right) + J \\ J = \displaystyle\int_0^{10}\!\!\int_0^{\pi/2} r\sqrt{1 + r^{\prime 2}(1 - \sin\theta - \cos\theta)^2}\,d\theta\,dz = 10.650609236799832,\quad r = 2\left(1 - \tfrac{z}{5}\right)^2 \end{array}'
+++

A cube of side 10 between the origin and (10, 10, 10). Round the edge where its faces x = 10 and
y = 10 meet, the line from (10, 10, 0) to (10, 10, 10), with a radius that falls to nothing in the
middle of the edge and grows again: r = 2(1 − z/5)², so 2 at the bottom face z = 0, 0 at the middle
z = 5 and 2 at the top face z = 10. At the middle the round has no width and the edge is sharp
there; the round is two horns, one from each end, meeting at that one point.

The round's sections square to the edge are circular: at each height z the round's section by the
horizontal plane is the quarter circle of radius r(z) centered at (10 − r, 10 − r, z), tangent to
the face x = 10 at (10, 10 − r, z) and to the face y = 10 at (10 − r, 10, z). It cuts the bottom and
top faces each with the quarter circle of radius 2.

**As a set:** with r = 2(1 − z/5)²,

$$[0,10]^3 \setminus \{\,x \ge 10 - r,\ y \ge 10 - r,\ (x - 10 + r)^2 + (y - 10 + r)^2 \ge r^2\,\}$$
