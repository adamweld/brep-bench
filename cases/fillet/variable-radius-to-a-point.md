+++
title = "A cube edge rounded with a radius falling to nothing at the corner"
tags = ["fillet", "variable-radius", "degenerate", "cone", "analytic"]
origin = "OCCT tests/blend/tolblend_buildvol/A1"

[volume]
value = 997.1386421786326
exact = "1000 − (1 − π/4)∫₀¹⁰ r² dz, r = 2 − z/5: 1000 − (40/3)(1 − π/4)"
tex = '1000 - \left(1 - \tfrac{\pi}{4}\right)\int_0^{10} r^2\,dz,\quad r = 2 - \tfrac{z}{5}:\ = 1000 - \tfrac{40}{3}\left(1 - \tfrac{\pi}{4}\right)'

[area]
value = 594.8778376869467
exact = "600 − 2∫₀¹⁰ r dz − (1 − π/4)·2² + ∫₀¹⁰ r dz·I = 580 − 4(1 − π/4) + 10I, I = ∫₀^{π/2} √(1 + (1 − sin θ − cos θ)²/25) dθ = 1.5736245033356860 (the round's area element is r√(1 + r′²(1 − sin θ − cos θ)²) dθ dz, r′ = −1/5)"
tex = '\begin{array}{l} 580 - 4\left(1 - \tfrac{\pi}{4}\right) + 10I \\ I = \displaystyle\int_0^{\pi/2}\sqrt{1 + \tfrac{(1 - \sin\theta - \cos\theta)^2}{25}}\,d\theta = 1.5736245033356860 \end{array}'
+++

A cube of side 10 between the origin and (10, 10, 10). Round the edge where its faces x = 10 and
y = 10 meet, the line from (10, 10, 0) to (10, 10, 10), with a radius that falls linearly along it
to nothing: r = 2 − z/5, so 2 at the bottom face z = 0 and 0 at the top face z = 10.

The round's sections square to the edge are circular: at each height z the round's section by the
horizontal plane is the quarter circle of radius r(z) centered at (10 − r, 10 − r, z), tangent to
the face x = 10 at (10, 10 − r, z) and to the face y = 10 at (10 − r, 10, z). So the round is part
of an oblique circular cone whose apex is the cube's corner (10, 10, 10), tangent to both faces
along straight lines from (10, 8, 0) and (8, 10, 0) to that corner. It cuts the bottom face with
the quarter circle of radius 2 and leaves the top face whole: the round, the two faces and the
top all meet at the one corner.

**As a set:**

$$[0,10]^3 \setminus \{\, x > 10 - r,\ y > 10 - r,\ (x - 10 + r)^2 + (y - 10 + r)^2 > r^2 \,\},\quad r = 2 - \tfrac{z}{5}$$
