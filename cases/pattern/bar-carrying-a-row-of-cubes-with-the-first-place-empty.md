+++
title = "A narrow bar carrying a row of four cubes"
tags = ["pattern", "linear", "suppressed", "union", "face-contact", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestLinearPattern.py::testSuppressOriginalFeatureOccurrence"

[volume]
value = 5440.0
exact = "60·6·4 + 4·10³"
tex = '60\cdot 6\cdot 4 + 4\cdot 10^3'

[area]
value = 2568.0
exact = "bar: bottom 360, long sides 2·60·4, ends 2·6·4, top left uncovered 360 − 240; cubes' row: top 400, bottom overhang 400 − 240, long sides 2·40·10, ends 2·10²: 360 + 480 + 48 + 120 + 160 + 400 + 800 + 200"
+++

A narrow bar with a row of cubes standing on it, overhanging it on both sides, the place at the bar's
end where the first cube would go left empty.

The bar is [0, 60] × [2, 8] × [−4, 0]. The cubes have side 10 and stand on its top face, z = 0 up to z = 10,
centred across it: four of them, [10i, 10i + 10] × [0, 10] × [0, 10] for i = 1, 2, 3, 4, touching one
another face to face in a row from x = 10 to x = 50. (The cells i = 0 and i = 5 are empty.) The cubes and
the bar are fused into one solid. Each cube overhangs the bar by 2 on both sides along y.

**As a set:**

$$[0, 60] \times [2, 8] \times [-4, 0] \ \cup\ [10, 50] \times [0, 10] \times [0, 10]$$
