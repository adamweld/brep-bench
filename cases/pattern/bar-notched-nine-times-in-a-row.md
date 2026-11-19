+++
title = "A bar notched nine times in a row, like a rack"
tags = ["pattern", "linear", "cut", "planar", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestLinearPattern.py::testSuppressOriginalSubtractiveOccurrence"

[volume]
value = 21408.0
exact = "100·12·20 − 9·24·12 = 21408: the bar less nine notches, each a trapezoid of area (6 + 2)/2·6 = 24 run 12 across"
tex = '100\cdot 12\cdot 20 - 9\cdot\tfrac{6 + 2}{2}\cdot 6\cdot 12 = 21408'

[area]
value = 7382.1039491927399
exact = "6880 + 9(−6·12 + 2·12 + 2·12√40 − 2·24) = 6016 + 432√10: the bar, and for each notch its mouth taken from the top and its trapezoid from each end, its floor and two slanting walls, each √(2² + 6²) = √40 down its slope"
tex = '6880 + 9\left(-6\cdot 12 + 2\cdot 12 + 2\cdot 12\sqrt{40} - 2\cdot 24\right) = 6016 + 432\sqrt{10}'
+++

A bar, the box from the origin to (100, 12, 20), with a row of nine notches cut across its top, a
linear pattern of one notch: like a rack's tooth spaces.

The notch runs right across the bar, along y from y = 0 to y = 12. Its section in the plane y = 0
is a trapezoid, symmetric about the line x = 10: 6 wide at the top face (z = 20), from x = 7 to
x = 13, narrowing to a flat floor 2 wide at z = 14, from x = 9 to x = 11. So it is 6 deep and its
walls slope 1 across for every 3 down.

The pattern repeats the notch along +x every 10: nine notches in all, centred on x = 10, 20, …, 90.
(A first notch at x = 0, on the bar's end, is not cut.) Neighbouring notches leave a flat land 4
wide on the top between them, and the bar's top is whole from x = 0 to 7 and from x = 93 to 100.

**As a set:**

$$\bigl([0,100]\times[0,12]\times[0,20]\bigr) \setminus \bigcup_{k=1}^{9} \{\, |x - 10k| \le 1 + \tfrac{z - 14}{3},\ z \ge 14 \,\}$$
