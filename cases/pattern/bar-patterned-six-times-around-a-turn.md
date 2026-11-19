+++
title = "A bar patterned six times around a turn, each copy lying on another"
tags = ["pattern", "polar", "union", "coincident", "planar", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestPolarPattern.py testZAxisPolarPattern; TestCircularPattern.py testCircularPatternCreatesConcentricCopies"

[volume]
value = 3132.3462819564033
exact = "h·(3·2L·2w − 3·(2w)²/sin 60° + 2√3w²), L = 20, w = 3, h = 5: the three bars, less each pair's overlap (a rhombus), plus the hexagon all three share: = 3600 − 270√3"
tex = 'h\left(3\cdot 2L\cdot 2w - 3\,\tfrac{(2w)^2}{\sin 60^\circ} + 2\sqrt{3}\,w^2\right),\quad L = 20,\ w = 3,\ h = 5:\ = 3600 - 270\sqrt{3}'

[area]
value = 2321.1693674201633
exact = "2·(720 − 54√3) + h·6(2w + 2(L − √3w)): the top and bottom, and six arms, each its end 2w and two sides L − √3w long = 2820 − 288√3"
tex = '2\left(720 - 54\sqrt{3}\right) + 6h\left(2w + 2\left(L - \sqrt{3}\,w\right)\right) = 2820 - 288\sqrt{3}'
+++

A bar patterned about the z axis: six copies, every 60° around a whole turn, joined into one solid.

The bar is the box −20 ≤ x ≤ 20, −3 ≤ y ≤ 3, 0 ≤ z ≤ 5. The copies are the bar itself and the bar
turned about the z axis by 60°, 120°, 180°, 240° and 300°.

The bar is symmetric under a half turn about the z axis, so each copy lies exactly on the one
three steps from it: the bar turned 180° is the bar again. The six copies make three distinct bars
crossing at the axis, a star of six arms, every point of it covered by two copies.

The star's section: six arms 6 wide, meeting in a regular hexagon of inradius 3 about the axis.
The facing sides of two neighbouring arms meet at an inner corner 6 from the axis, on the line
midway between them. So each side of an arm runs 20 − 3√3 from that corner out to the arm's end,
which is flat, 20 from the axis.

**As a set:**

$$\bigcup_{k=0}^{5} R_z(60^\circ k)\, B, \quad B = [-20, 20] \times [-3, 3] \times [0, 5]$$
