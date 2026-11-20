+++
title = "A grid of touching cubes with two cells missing"
tags = ["pattern", "linear", "suppressed", "union", "face-contact", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestLinearPattern.py::testSuppressionKeepsGridPosition; src/Mod/PartDesign/PartDesignTests/TestLinearPattern.py::testSuppressionComposesAcrossHelpers; src/Mod/PartDesign/PartDesignTests/TestLinearPattern.py::testXAxisLinearPattern; src/Mod/PartDesign/PartDesignTests/TestLinearPattern.py::testSuppressOriginalWholeShapeOccurrence"

[volume]
value = 10000.0
exact = "10 cubes of 10³"
tex = '10 \cdot 10^3'

[area]
value = 3800.0
exact = "top and bottom 2·10·100, and the outline of the ten cells, 18 cell edges of 10 by height 10: 2000 + 1800 (outline 14 edges of the 4 × 3 grid, the notched corner keeps it at 14, the hole adds 4)"
tex = '2000 + 18\cdot 10\cdot 10'
+++

A flat grid of cubes, touching face to face, with two cells missing.

The grid is four cells along x and three along y, each cell a cube of side 10: the cell in column i and
row j (i = 0 to 3, j = 0 to 2) is [10i, 10i + 10] × [10j, 10j + 10] × [0, 10]. Twelve cubes would make a
40 × 30 × 10 slab; here the cubes in cells (0, 0) and (2, 1) are left out: the first is the corner at the
origin (where a pattern's original instance would be) and the second is in the middle of the grid, so the
slab has a notched corner and a square hole through it. The ten cubes left are fused into one solid
(neighbours share whole faces; no two touch along only an edge).

**As a set:**

$$\bigl([0, 40] \times [0, 30] \setminus \bigl([0, 10]^2 \cup [20, 30] \times [10, 20]\bigr)\bigr) \times [0, 10]$$
