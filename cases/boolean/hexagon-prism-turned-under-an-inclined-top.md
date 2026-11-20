+++
title = "A hexagonal prism turned 15°, under an inclined top"
tags = ["boolean", "cut", "analytic", "planar", "oblique"]
origin = "OCCT boolean/volumemaker/A5, OCCT boolean/volumemaker/B7, OCCT boolean/volumemaker/B8, OCCT boolean/volumemaker/B9"

[volume]
value = 443.40500673763259
exact = "A·8, A = 2√3·a² = 32√3 (a = 4, the hexagon's apothem); the top's tilt adds nothing, since the hexagon is centered under it"
tex = '8A,\quad A = 2\sqrt3\,a^2 = 32\sqrt3'

[area]
value = 334.25955663936317
exact = "A + A√17/4 + 6·s·8, s = 2a/√3 (the six sides' heights average 8 each, by symmetry)"
tex = 'A + \tfrac{\sqrt{17}}{4}A + 6\cdot 8s,\quad s = \tfrac{2a}{\sqrt3}'
+++

A regular hexagonal prism standing on the plane z = 0, eight planes in all. Its six side faces are the planes at distance 4 from the z axis whose outward normals point at 15°, 75°, 135°, 195°, 255° and 315° from +x toward +y, so that none is parallel to an axis plane. Its floor is z = 0 and its top is the inclined plane z = 8 + 0.25x, 8 high over the axis.

**As a set:**

$$\{\,x\cos\alpha_k + y\sin\alpha_k \le 4\ (\alpha_k = 15^\circ + 60^\circ k,\ k = 0, \dots, 5),\ 0 \le z \le 8 + 0.25x\,\}$$
