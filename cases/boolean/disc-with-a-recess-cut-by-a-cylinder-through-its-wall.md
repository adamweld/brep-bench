+++
title = "A disc with a round recess, cut by a cylinder that overlaps the recess and pokes out of the disc wall"
tags = ["boolean", "cut", "analytic", "cylinder", "planar", "coincident"]
origin = "OCCT boolean/bcut_simple/L3"

[volume]
value = 1154.2219418204547
exact = "4πR² − A_F·1, R = 10; A_F = πr² + L(5, 10; 8) − L(4, 5; 8), the footprint of the recess and the tool inside the disc, L the two-circle lens area"
tex = '4\pi R^2 - A_F,\quad A_F = \pi r^2 + L(5,10;8) - L(4,5;8)'

[area]
value = 902.99512414285718
exact = "2πR² − A_F + 2πR·4 − 2Rγ + A_F + r(2π − 2α) + 5·2(φ_d − φ_r), γ = arccos(139/160), α = arccos(55/64), φ_d = arccos(−11/80), φ_r = arccos(73/80): the two faces (the top less the footprint), the disc wall less the arc the tool opens, the floor, the recess wall outside the tool and the tool wall inside the disc and outside the recess, each of the last three 1 high"
tex = '2\pi R^2 - A_F + 8\pi R - 2R\gamma + A_F + r(2\pi - 2\alpha) + 10(\varphi_d - \varphi_r)'
+++

A disc: the cylinder of radius 10 about the z axis from z = 0 to z = 4. Cut from it, one after the other, two flat-bottomed cylinders that both begin at the plane z = 3 and run up past the disc's top face: first the recess, of radius 4 about the z axis, then the tool, of radius 5 about the axis parallel to z through (8, 0). The tool's axis is 8 from the disc's, less than the 9 that the tool's and the recess's radii add, so it overlaps the recess part way; and the tool's far side reaches 13 from the disc's axis, so it pokes out of the disc's wall, taking a notch there. The recess is 1 deep, the tool takes 1 of the disc's thickness as well, and the two floors are one plane, z = 3.

The result has one floor, in z = 3, over the two circles together inside the disc, with the recess wall outside the tool, the tool wall inside the disc and outside the recess, and the disc's wall notched where the tool crosses it.

**As a set:**

$$\{\rho \le 10,\ 0 \le z \le 4\} \setminus \bigl(\{\rho \le 4,\ z \ge 3\} \cup \{(x-8)^2 + y^2 \le 25,\ z \ge 3\}\bigr)$$
