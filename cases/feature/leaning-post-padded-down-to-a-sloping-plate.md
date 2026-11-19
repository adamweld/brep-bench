+++
title = "A leaning square post extruded aslant down to a sloping plate"
tags = ["feature", "extrude", "up-to-face", "oblique", "union", "planar", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestPad.py::testPadToPlaneCustomDir; src/Mod/PartDesign/PartDesignTests/TestPad.py::testPadToFaceCase"

[volume]
value = 17873.684210526316
exact = "40·60·6 + 2·10²·s(10) = 14400 + 66000/19, s(x₀) = (34 − x₀/10)/1.9: the plate (its slope adds nothing over a span symmetric about x = 0), and the post, the square slid s along (−1, 0, −2), which sweeps volume 2 per unit area per unit s, s linear across the square so its mean is s at the centre x₀ = 10"
tex = '40\cdot 60\cdot 6 + 2\cdot 10^2\, s(10) = 14400 + \tfrac{66000}{19},\quad s(x_0) = \frac{34 - x_0/10}{1.9}'

[area]
value = 7477.6582294518786
exact = "2400 + 480 + 720 + 240√101 (the plate: bottom, ends 40·3 + 40·9, sides 2·360, sloping top) − 200√101/19 (the post's foot on it) + 100 (the post's top) + 2·20·s(10) (its two sides square to y, parallelograms 2·∫s dx₀) + 10√5(s(5) + s(15)) (its two slanting sides) = 3700 + 13200/19 + 4360√101/19 + 6600√5/19"
tex = '2400 + 480 + 720 + 240\sqrt{101} - \tfrac{200\sqrt{101}}{19} + 100 + \tfrac{13200}{19} + \tfrac{6600\sqrt5}{19} = 3700 + \tfrac{13200 + 4360\sqrt{101} + 6600\sqrt5}{19}'
+++

A plate whose top slopes, and a square post on a plane above it extruded aslant, down until it
meets the plate's top face, and joined to it.

The plate: the box −30 ≤ x ≤ 30, −20 ≤ y ≤ 20, z ≥ 0 cut off above by the plane z = 6 + x/10,
so it is 3 thick at its end x = −30 and 9 thick at x = 30; its sloping top is a rectangle 40
wide.

The post: the square 5 ≤ x ≤ 15, −5 ≤ y ≤ 5 in the plane z = 40, extruded along the direction
(−1, 0, −2), down and leaning toward −x, not square to the square's plane, until it meets the
plate's top face. Each point (x₀, y₀, 40) of the square runs to (x₀ − s, y₀, 40 − 2s) with
s = (34 − x₀/10)/1.9, where the line meets the plate's top: the square's centre lands at about
(−7.368, 0, 5.263). The post's foot is a parallelogram on the sloping top, wholly inside it, and
its top is the square at z = 40. Its sides square to y are parallelograms in the planes y = ±5;
its other two sides slant, along y and along the direction of extrusion.

**As a set:** with P the plate,

$$P \cup \{\, 5 \le x + \tfrac{40 - z}{2} \le 15,\ |y| \le 5,\ 6 + \tfrac{x}{10} \le z \le 40 \,\}$$
