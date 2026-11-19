+++
title = "A square pad extruded out to the inside of a half ring"
tags = ["feature", "extrude", "up-to-face", "cylinder", "concave", "union", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestPad.py testPadToConcaveCase"

[volume]
value = 5288.958902465079
exact = "π(26² − 20²)·10/2 + 6S, S = ∫₋₄⁴ √(20² − x²) dx = 4√384 + 400 arcsin(1/5) = 32√6 + 400 arcsin(1/5): = 1380π + 6S"
tex = '\tfrac{1}{2}\pi\left(26^2 - 20^2\right)\cdot 10 + 6S,\quad S = \int_{-4}^{4}\sqrt{20^2 - x^2}\,dx = 32\sqrt{6} + 400\arcsin\tfrac{1}{5}'

[area]
value = 2984.8909875299814
exact = "the half ring 2·60 + 276π + 260π + 200π, less its inner side under the pad 20·2 arcsin(1/5)·6, plus the pad's top and bottom 2S, its sides 2·6·8√6 and its start 8·6: = 168 + 736π + 160√6 + 560 arcsin(1/5)"
tex = '120 + 736\pi - 240\arcsin\tfrac{1}{5} + 2S + 96\sqrt{6} + 48 = 168 + 736\pi + 160\sqrt{6} + 560\arcsin\tfrac{1}{5}'
+++

A half ring, and a square pad that runs out from near its axis until it meets the ring's inside.

The half ring: the rectangle 20 ≤ x ≤ 26, 0 ≤ z ≤ 10 in the plane y = 0, revolved half a turn
about the z axis from +x toward +y. So it is everything between 20 and 26 from the z axis with
y ≥ 0 and 0 ≤ z ≤ 10, its two flat ends in the plane y = 0.

The pad: the square |x| ≤ 4, 2 ≤ z ≤ 8 in the plane y = 0 (in the ring's opening, clear of its
ends), extruded along +y up to the first face it meets, the ring's inner side, the cylinder of
radius 20 about the z axis, and joined to the ring.

So the pad's four flat sides run from y = 0 out to that cylinder, which they reach at
y = √(20² − x²): 20 at the square's middle, x = 0, and √384 = 8√6 ≈ 19.596 at its sides x = ±4.
The pad's far end is the patch of the cylinder in the square's path, which it shares with the ring.
The pad and the ring are one solid.

**As a set:**

$$\{\,20 \le \rho \le 26,\ y \ge 0,\ 0 \le z \le 10\,\} \cup \{\,|x| \le 4,\ 2 \le z \le 8,\ y \ge 0,\ \rho \le 20\,\}$$
