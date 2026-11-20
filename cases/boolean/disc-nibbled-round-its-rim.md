+++
title = "A disc nibbled round its rim, 24 bites"
tags = ["boolean", "cut", "analytic", "sequence", "cylinder"]
origin = "OCCT boolean/simplify/A3"

[volume]
value = 4109.0802275754808
exact = "T(πR² − 24L), L = lens of the rim circle (R = 20) and a bite circle (r = 2.5, centered on the rim), by the two-circle lens formula; T = 4"
tex = 'T\left(\pi R^2 - 24L\right),\quad L = r^2\theta_0 + R^2\gamma - \tfrac12\sqrt{(r+R-d)(d+r-R)(d-r+R)(d+r+R)},\ d = R,\ \theta_0 = \arccos\tfrac{r}{2d},\ \gamma = \arccos\bigl(1 - \tfrac{r^2}{2R^2}\bigr)'

[area]
value = 2800.844558964288
exact = "2(πR² − 24L) + (2πR − 24·2Rγ)T + 24·2rθ₀T"
tex = '2\left(\pi R^2 - 24L\right) + \left(2\pi R - 24\cdot 2R\gamma\right)T + 24\cdot 2r\theta_0 T'
+++

A disc of radius 20 and thickness 4: its axis is the z axis and it runs from z = 0 to z = 4. Nibble its rim 24 times, each bite its own cut, one after another. The k-th bite (k = 0, 1, … 23) is a cylinder of radius 2.5 whose axis is parallel to z and passes through the point (20 cos 15°k, 20 sin 15°k, 0) on the rim, long enough to run past both faces of the disc. Neighboring axes are 5.22 apart, more than the 5 that two bites reach, so the bites do not touch each other. Each takes a lens-shaped bite out of the rim.

**As a set:**

$$\{\,\rho \le 20,\ 0 \le z \le 4\,\} \setminus \bigcup_{k=0}^{23} \{\,(x - 20\cos 15^\circ k)^2 + (y - 20\sin 15^\circ k)^2 \le 2.5^2\,\}$$
